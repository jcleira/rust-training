fn main() {
    println!("Hello, world!");

    another_function(3, 'c');

    println!("yet_another_function {}", yet_another_function());
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function print {} {}", value, unit_label);
}

fn yet_another_function() -> i32 {
    5
}
