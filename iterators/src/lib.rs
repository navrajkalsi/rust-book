#[derive(PartialEq, Debug)]
struct _Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_size(shoes: Vec<_Shoe>, shoe_size: u32) -> Vec<_Shoe> {
    // this function takes the ownership of the input vector
    // we call into_iter() to create an iterator that takes the ownership of the vector and then
    // filter to create a new iterator, and then collect the new one and return the collection
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            _Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            _Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            _Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = _shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                _Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                _Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
