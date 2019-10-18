fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}


fn main() -> () {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let result = largest_ref(&number_list);

    println!("The largest number is {}", result);

    let result = largest_clone(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);

    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);
}
