fn main() {
    let s = "hello";

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let x1 = 5;
    makes_copy(x);
    println!("x = {}, y = {}, x1 = {}", x, y, x1);

    let mut s3 = String::from("hello");
    // takes_ownership(s3); This would make the next line to fail.
    s3 = takes_and_returns_ownership(s3);

    println!("{}, world!", s3);

    let s4 = String::from("hello");
    takes_ownership(s4);

    let mut s5 = String::from("hello");
    send_reference(&s5);
    send_and_update_reference(&mut s5);

    let slice_word = String::from("good morning");
    let slice_word_slice = &slice_word[..5];
    println!("{}", slice_word_slice);

    let result = first_word(&slice_word);
    println!("{}", result);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_returns_ownership(some_string: String) -> String {
    some_string
}

fn send_reference(some_string: &String) {
    println!("{}", some_string);
}

fn send_and_update_reference(some_string: &mut String) {
    some_string.push_str("!");
    println!("{}", some_string);
}

fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[..i];
        }
    }

    &some_string[..]
}
