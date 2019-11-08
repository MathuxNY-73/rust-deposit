use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention announce: {}", announcement);
        self.part
    }

    fn announce_and_return_part_t<T>(&self, announcement: T) -> &str
        where T: Display
    {
        println!("Attention announce: {}", announcement);
        self.part
    }
}

fn main() {
    test_lifetime_longest_right();
    test_struct_lifetime();
    test_lifetime_longest_wrong();
    test_struct_method_lifetime();
    test_lifetime_longest_right_with_announce();
    test_struct_method_lifetime_with_t();
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}

fn longest_with_announce<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Attention announce: {}", ann);
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}


fn test_lifetime_longest_right() -> () {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn test_lifetime_longest_right_with_announce() -> () {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_announce(string1.as_str(), string2, 3);
    println!("The longest string is {}", result);
}


fn test_lifetime_longest_wrong() -> () {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // Uncomment line below to get compiler error
    //println!("The longest string is {}", result);
}

fn test_struct_lifetime() -> () {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    println!("The first sentence of the novel is: {}", i.part);
}

fn test_struct_lifetime_wrong() -> () {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        i = ImportantExcerpt { part: first_sentence };
    }

    // Uncomment line below to get compiler error
    //println!("The first sentence of the novel is: {}", i.part);
}

fn test_struct_method_lifetime() -> () {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    let p = i.announce_and_return_part(String::from("Hello World!").as_str());

    println!("The first sentence of the novel is: {}", p);
}

fn test_struct_method_lifetime_with_t() -> () {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    let p = i.announce_and_return_part_t(String::from("Hello World!").as_str());

    println!("The first sentence of the novel is: {}", p);
}

