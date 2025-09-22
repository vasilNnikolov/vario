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

unsafe impl<T, const N: usize> Sync for Queue<T, N> where T: Send {}
unsafe impl<T, const N: usize> Send for Queue<T, N> where T: Send {}

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

    pub fn pop_ref(&self) -> QueueResult<Option<&T>> {
        unsafe { self.with_lock(|this| this.pop_unchecked()) }
    }

    pub fn pop(&self) -> QueueResult<Option<T>> {
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
    /// returns `None` if the push succeeded, `Some(x)` if it failed (Queue full)
    unsafe fn push_unchecked(&self, x: T) -> Option<T> {
        unsafe {
            let len = *self.len.get();
            if len < N {
                *self.storage[(*self.head.get() + *self.len.get()) % N].get() = MaybeUninit::new(x);
                *self.len.get() += 1;
                None
            } else if len == N {
                Some(x)
            } else {
                panic!(
                    "Logical error (runtime length {} bigger than the constant length {})",
                    len, N
                );
            }
        }
    }

    /// Returns `Ok(None)` if the push succeeded, `Ok(Some(x))` if the push failed because the queue is full, and `Err` otherwise
    pub fn push(&self, x: T) -> QueueResult<Option<T>> {
        unsafe { self.with_lock(|this| this.push_unchecked(x)) }
    }
}

impl<T: Send + Copy, const N: usize> Queue<T, N> {
    fn pop_full(&self) -> QueueResult<Option<T>> {
        self.pop_ref().map(|x| x.map(|r| r.clone()))
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::*;
    extern crate std;

    #[test]
    fn no_push() {
        let q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.pop_ref(), Ok(None));
    }

    #[test]
    fn push_pop_no_overflow() {
        let q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.push(0), Ok(None));
        assert_eq!(q.push(1), Ok(None));
        assert_eq!(q.push(2), Ok(None));
        assert_eq!(q.pop_full(), Ok(Some(0)));
        assert_eq!(q.pop_full(), Ok(Some(1)));
        assert_eq!(q.pop_full(), Ok(Some(2)));
        assert_eq!(q.pop_full(), Ok(None));
    }

    #[test]
    fn push_pop_overflow() {
        let q: Queue<u32, 5> = Queue::new();
        assert_eq!(q.push(0), Ok(None));
        assert_eq!(q.push(1), Ok(None));
        assert_eq!(q.push(2), Ok(None));
        assert_eq!(q.push(3), Ok(None));
        assert_eq!(q.push(4), Ok(None));
        assert_eq!(q.push(5), Ok(Some(5))); // tries to overflow the queue
        assert_eq!(q.push(55), Ok(Some(55)));

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
    /// thread index `i` sends numbers `i*K` to `(i+1)*K`. In the end,the receiver should receive all numbers from `0` to `N*K`
    fn stress_test() {
        use std::thread;
        use std::vec::Vec;
        const N_THREADS: u32 = 10;
        const K: u32 = 1000;
        let q: Queue<u32, 100> = Queue::new();
        let refq = &q;
        thread::scope(|s| {
            let mut handles: Vec<_> = Vec::new();
            for i in 0..N_THREADS {
                let handle = s.spawn(move || {
                    for num in i * K..(i + 1) * K {
                        loop {
                            match refq.push(num) {
                                Ok(None) => break,
                                _ => {
                                    // avoid busy looping
                                    thread::sleep(Duration::from_millis(1));
                                }
                            }
                        }
                    }
                });
                handles.push(handle);
            }

            // create consumer thread
            s.spawn(|| {
                loop {
                    match refq.pop_ref()
                }
            })

            for h in handles {
                h.join();
            }
        })
    }
}
