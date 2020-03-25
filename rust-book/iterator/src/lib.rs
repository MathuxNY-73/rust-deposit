mod counter;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
         .filter(move |s| s.size == my_size)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }
}
