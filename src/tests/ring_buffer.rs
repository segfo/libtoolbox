#[cfg(test)]
mod enqueue_tests {
    use crate::ring_buffer::RingBuffer;
    #[test]
    fn enqueue_1() {
        let mut buf = RingBuffer::new(3, 0);
        buf.enqueue(10);
        buf.enqueue(20);
        assert_eq!(buf.dequeue(), Some(10));
    }
    #[test]
    fn enqueue_2() {
        let mut buf = RingBuffer::new(3, 0);
        for i in 0..6 {
            buf.enqueue(i);
        }
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.dequeue(), Some(3));
        assert_eq!(buf.len(), 2);
    }
    #[test]
    fn enqueue_3() {
        let mut buf = RingBuffer::new(3, 0);
        for i in 0..6 {
            buf.enqueue(i);
        }
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.dequeue(), Some(3));
        assert_eq!(buf.dequeue(), Some(4));
        assert_eq!(buf.dequeue(), Some(5));
        assert_eq!(buf.len(), 0);
    }
    #[test]
    fn enqueue_4() {
        let mut buf = RingBuffer::new(3, 0);
        for i in 0..6 {
            buf.enqueue(i);
        }
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.dequeue(), Some(3));
        assert_eq!(buf.dequeue(), Some(4));
        assert_eq!(buf.len(), 1);
        buf.enqueue(10);
        buf.enqueue(20);
        assert_eq!(buf.dequeue(), Some(5));
        assert_eq!(buf.dequeue(), Some(10));
        assert_eq!(buf.dequeue(), Some(20));
    }
}
#[cfg(test)]
mod dequeue_tests {
    use crate::ring_buffer::RingBuffer;
    #[test]
    fn dequeue_1() {
        let mut buf = RingBuffer::new(3, 0);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.dequeue(), None);
        for i in 0..3 {
            buf.enqueue(i);
        }
        assert_eq!(buf.len(), 3);
        for _i in 0..buf.len() {
            assert!(buf.dequeue().is_some(), "Expected Some<V>");
        }
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.dequeue(), None);
    }
    #[test]
    fn dequeue_2() {
        let mut buf = RingBuffer::new(3, 0);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.dequeue(), None);
        let expect = [[1, 2, 3], [2, 3, 4], [3, 4, 5], [4, 5, 6]];
        for x in 0..3 {
            for i in 0..4 {
                buf.enqueue(i + x);
            }
            assert_eq!(buf.len(), 3);
            for i in 0..buf.len() {
                assert_eq!(buf.dequeue(), Some(expect[x][i]));
            }
            assert_eq!(buf.len(), 0);
            assert_eq!(buf.dequeue(), None);
        }
    }
}
