#![no_std]
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
// use std::sync::atomic::AtomicBool;
use core::sync::atomic::{AtomicBool, Ordering};

/// MPMC queue with interior mutability
pub struct Queue<T: Send, const N: usize> {
    storage: [UnsafeCell<MaybeUninit<T>>; N],
    head: UnsafeCell<usize>,
    len: UnsafeCell<usize>,
    in_use: AtomicBool,
}

pub enum QueueErr {
    QueueInUse,
}
type QueueResult<T> = Result<T, QueueErr>;

impl<T: Send + Copy, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Queue {
            storage: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
            head: UnsafeCell::new(0),
            len: UnsafeCell::new(0),
            in_use: AtomicBool::new(false),
        }
    }

    /// tries to perform the operation `function` with the queue locked. If it succeeds, returns the result of the function in an OK
    /// if the lock fails, returns Err
    fn with_lock<'a, F, R>(&'a self, function: F) -> QueueResult<R>
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
                Err(QueueErr::QueueInUse)
            }
        }
    }

    /// SAFETY: do not run if the queue is not locked
    fn pop_unchecked(&self) -> Option<&T> {
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
        self.with_lock(Queue::pop_unchecked)
    }

    /// get a pointer to the fist pushed element
    /// SAFETY: do not call if there is no such element, or if the queue is not locked
    unsafe fn head_ptr(&self) -> *mut T {
        unsafe {
            let x = *(self.storage[*self.head.get()].get());
            &mut x.assume_init() as *mut T
        }
    }

    pub fn push(&mut self, x: T) -> Result<(), ()> {
        unsafe {
            if *self.len.get() < N {
                *self.storage[(*self.head.get() + *self.len.get()) % N].get() = MaybeUninit::new(x);
                *self.len.get() += 1;
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

// impl<T: Send + Copy, const N: usize> Queue<T, N> {
//     fn pop_full(&self) -> Option<T> {
//         match self.pop() {
//             Some(&x) => Some(x.clone()),
//             None => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::Queue;

//     #[test]
//     fn test_no_push() {
//         let q: Queue<u32, 5> = Queue::new();
//         assert!(q.pop().is_none());
//     }

//     #[test]
//     fn test_push_pop_no_overflow() {
//         let mut q: Queue<u32, 5> = Queue::new();
//         assert!(q.push(0).is_ok());
//         assert!(q.push(1).is_ok());
//         assert!(q.push(2).is_ok());
//         assert_eq!(q.pop_full(), Some(0));
//         assert_eq!(q.pop_full(), Some(1));
//         assert_eq!(q.pop_full(), Some(2));
//         assert_eq!(q.pop_full(), None);
//     }

//     #[test]
//     fn test_push_pop_overflow() {
//         let mut q: Queue<u32, 5> = Queue::new();
//         assert!(q.push(0).is_ok());
//         assert!(q.push(1).is_ok());
//         assert!(q.push(2).is_ok());
//         assert!(q.push(3).is_ok());
//         assert!(q.push(4).is_ok());
//         assert!(q.push(5).is_err()); // tries to overwrite 0

//         assert_eq!(q.pop_full(), Some(0));
//         assert_eq!(q.pop_full(), Some(1));
//         assert_eq!(q.pop_full(), Some(2));
//         assert!(q.push(1234).is_ok());
//         assert_eq!(q.pop_full(), Some(3));
//         assert_eq!(q.pop_full(), Some(4));
//         assert_eq!(q.pop_full(), Some(1234));
//         assert!(q.pop_full().is_none());
//         assert!(q.pop_full().is_none());
//     }
// }
