#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1)); // calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. Basically, we consume the iterator with each call to next. Something important to know is that the values we get from the calls to next are immutable references to the values in the vector. If we want to create an iterator that takes ownership and returns owned values, we can use `into_iter` instead of just `iter`. Also, if we want to iterate over mutable references, we can call `iter_mut`.
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
