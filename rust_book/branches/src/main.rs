fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("again!");

        if count == 10 {
            break 'counting_up;
        }

        count += 1;
    }

    let a: [i32; 3] = [5, 0, 3];

    for element in a {
        println!("the value is: {}", element);
    }
}
