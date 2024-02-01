
#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // The filter closure gets an item from the iterator and returns a bool, thus determining if the provided value will be included in the iteration produced by filter.

    // by using into_iter(), we take ownership  of a vector of shoes and a she size, and returns a vector containing only shoes that fit our criteria.  Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1)); // calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. Basically, we consume the iterator with each call to next. Something important to know is that the values we get from the calls to next are immutable references to the values in the vector. If we want to create an iterator that takes ownership and returns owned values, we can use `into_iter` instead of just `iter`. Also, if we want to iterate over mutable references, we can call `iter_mut`.
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 13,
                style: String::from("sandal"),
            },
            Shoe{
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

