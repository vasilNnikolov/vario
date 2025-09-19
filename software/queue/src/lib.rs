#![no_std]
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::panic;
// use std::sync::atomic::AtomicBool;
use core::sync::atomic::{AtomicBool, Ordering};

/// MPMC queue with interior mutability
pub struct Queue<T: Send, const N: usize> {
    storage: [UnsafeCell<MaybeUninit<T>>; N],
    head: UnsafeCell<usize>,
    len: UnsafeCell<usize>,
    in_use: AtomicBool,
}

unsafe impl<T: Send, const N: usize> Sync for Queue<T, N> {}

#[derive(Debug, PartialEq, Eq)]
pub enum QueueErrKind {
    QueueInUse,
}
type QueueResult<T> = Result<T, QueueErrKind>;

impl<T: Send + Copy, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Queue {
            storage: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
            head: UnsafeCell::new(0),
            len: UnsafeCell::new(0),
            in_use: AtomicBool::new(false),
        }
    }

    /// tries to perform the operation `function` with the queue locked. If it succeeds, returns the result of the function in an Result::Ok
    /// if the lock fails, returns Result::Err
    /// **SAFETY**: the passed `function` should NOT mess with `self.in_use`
    unsafe fn with_lock<'a, F, R>(&'a self, function: F) -> QueueResult<R>
    where
        F: Fn(&'a Self) -> R,
        R: 'a,
    {
        match self
            .in_use
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(result) => {
                assert_eq!(result, false, "Logical error");
                let res = function(&self);

                // unlock the queue
                // TODO reason if this can be relaxed
                self.in_use.store(false, Ordering::Release);

                Ok(res)
            }
            Err(result) => {
                assert_eq!(result, true, "Logical error");
                Err(QueueErrKind::QueueInUse)
            }
        }
    }

    /// **SAFETY**: do not run if the queue is not locked
    unsafe fn pop_unchecked(&self) -> Option<&T> {
        let len = unsafe { *self.len.get() };
        if len == 0 {
            None
        } else {
            unsafe {
                let ret_value = &(*self.head_ptr());
                *self.len.get() -= 1;
                *self.head.get() += 1;
                *self.head.get() %= N;
                Some(ret_value)
            }
        }
    }

    pub fn pop(&self) -> QueueResult<Option<&T>> {
        unsafe { self.with_lock(|this| this.pop_unchecked()) }
    }

    /// get a pointer to the fist pushed element
    /// **SAFETY**: do not call if there is no such element, or if the queue is not locked
    unsafe fn head_ptr(&self) -> *mut T {
        unsafe {
            let x = *(self.storage[*self.head.get()].get());
            &mut x.assume_init() as *mut T
        }
    }

    /// **SAFETY**: do not call if queue is not locked
    unsafe fn push_unchecked(&self, x: T) -> bool {
        unsafe {
            let len = *self.len.get();
            if len < N {
                *self.storage[(*self.head.get() + *self.len.get()) % N].get() = MaybeUninit::new(x);
                *self.len.get() += 1;
                true
            } else if len == N {
                false
            } else {
                panic!(
                    "Logical error (runtime length {} bigger than the constant length {})",
                    len, N
                );
            }
        }
    }

    /// the result is true if the push succeeded, false if it didn't ()
    pub fn push(&self, x: T) -> QueueResult<bool> {
        unsafe { self.with_lock(|this| this.push_unchecked(x)) }
    }
}

impl<T: Send + Copy, const N: usize> Queue<T, N> {
    ///
    fn pop_full(&self) -> QueueResult<Option<T>> {
        self.pop().map(|x| x.map(|r| r.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn no_push() {
        let q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.pop(), Ok(None));
    }

    #[test]
    fn push_pop_no_overflow() {
        let mut q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.push(0), Ok(true));
        assert_eq!(q.push(1), Ok(true));
        assert_eq!(q.push(2), Ok(true));
        assert_eq!(q.pop_full(), Ok(Some(0)));
        assert_eq!(q.pop_full(), Ok(Some(1)));
        assert_eq!(q.pop_full(), Ok(Some(2)));
        assert_eq!(q.pop_full(), Ok(None));
    }

    #[test]
    fn push_pop_overflow() {
        let mut q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.push(0), Ok(true));
        assert_eq!(q.push(1), Ok(true));
        assert_eq!(q.push(2), Ok(true));
        assert_eq!(q.push(3), Ok(true));
        assert_eq!(q.push(4), Ok(true));
        assert_eq!(q.push(5), Ok(false)); // tries to overflow the queue
        assert_eq!(q.push(55), Ok(false));

        assert_eq!(q.pop_full(), Ok(Some(0)));
        assert_eq!(q.pop_full(), Ok(Some(1)));
        assert_eq!(q.pop_full(), Ok(Some(2)));
        assert!(q.push(1234).is_ok());
        assert_eq!(q.pop_full(), Ok(Some(3)));
        assert_eq!(q.pop_full(), Ok(Some(4)));
        assert_eq!(q.pop_full(), Ok(Some(1234)));
        assert_eq!(q.pop_full(), Ok(None));
    }

    #[test]
    /// spawn N threads, all attempting to make accesses to the same queue
    /// TODO think how to stress test for correctness
    fn stress_test() {
        use std::thread;
        let N_THREADS = 10;
        let q: Queue<u32, 100> = Queue::new();
        thread::scope(|s| {
            for _ in 0..N_THREADS {
                let handle = s.spawn(|| {
                    let _ = q.push(5);
                });
            }
        })
    }
}
