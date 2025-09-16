use core::mem::MaybeUninit;

pub struct Queue<T: Send, const N: usize> {
    storage: [MaybeUninit<T>; N],
    head: usize,
    len: usize,
}

impl<T: Send + Copy, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Queue {
            storage: [const { MaybeUninit::uninit() }; N],
            head: 0,
            len: 0,
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let r = unsafe { self.storage[self.head].assume_init() };
            self.head += 1;
            self.head %= N;
            self.len -= 1;
            Some(r)
        }
    }
    pub fn push(&mut self, x: T) -> Result<(), ()> {
        if self.len < N {
            self.storage[(self.head + self.len) % N] = MaybeUninit::new(x);
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Queue;

    #[test]
    fn test_no_push() {
        let mut q: Queue<u32, 5> = Queue::new();
        assert!(q.pop().is_none());
    }

    #[test]
    fn test_push_pop_no_overflow() {
        let mut q: Queue<u32, 5> = Queue::new();
        q.push(0);
        q.push(1);
        q.push(2);
        assert_eq!(q.pop(), Some(0));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert!(q.pop().is_none());
    }
    #[test]
    fn test_push_pop_overflow() {
        let mut q: Queue<u32, 5> = Queue::new();
        assert!(q.push(0).is_ok());
        assert!(q.push(1).is_ok());
        assert!(q.push(2).is_ok());
        assert!(q.push(3).is_ok());
        assert!(q.push(4).is_ok());
        assert!(q.push(5).is_err()); // tries to overwrite 0

        assert_eq!(q.pop(), Some(0));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert!(q.push(1234).is_ok());
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), Some(4));
        assert_eq!(q.pop(), Some(1234));
        assert!(q.pop().is_none());
        assert!(q.pop().is_none());
    }
}
