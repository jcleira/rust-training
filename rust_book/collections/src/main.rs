use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let v: Vec<i32> = Vec::new();

    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5, 6];

    println!("{:?}", v);

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{:?}", v);
    println!("{:?}", v[1]);
    println!("{:?}", v.get(2));

    match v.get(3) {
        Some(third) => println!("{:?}", third),
        None => println!("element not found"),
    }

    match v.get(100) {
        Some(third) => println!("{:?}", third),
        None => println!("element not found"),
    }

    for i in v {
        println!("{}", i)
    }

    #[derive(Debug)]
    enum SpreadCellType {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut sct: Vec<SpreadCellType> = Vec::new();

    sct.push(SpreadCellType::Int(3));
    sct.push(SpreadCellType::Float(4.0));
    sct.push(SpreadCellType::Text(String::from("hello")));

    for sct_val in sct {
        println!("{:?}", sct_val)
    }

    // Final exercices
    //
    // Find the middle value:
    let mut i_key = vec![1, 5, 6, 7, 8, 20, 1, 7, 7, 33];
    i_key.sort();
    let middle_value = i_key.get(i_key.len() / 2 + 1);
    println!("middle value of {:?} is: {:?}", i_key, middle_value);

    // Find the value with most occurrences
    let mut i_hash = HashMap::new();
    for i in i_key {
        let count = i_hash.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", i_hash);

    let mut max_value = 0;
    let mut max_value_key = 0;

    for (key, value) in i_hash {
        if value > max_value {
            max_value = value;
            max_value_key = key;
        }
    }
    println!(
        "The value with most ocurrences is {:?} with {:?}",
        max_value_key, max_value
    );

    // Modify strings
    //
    let str1 = String::from("apple");
    let mut str2 = String::new();
    let mut suffix = String::new();
    let mut first: bool = true;

    for c in str1.chars() {
        match first {
            true => {
                first = false;
                suffix = match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        str2.push(c);
                        String::from("-hay")
                    }
                    _ => format!("-{}ay", c),
                };
            }
            false => {
                str2.push(c);
            }
        }
    }

    println!("{}{}", str2, suffix);

    // HashMap
    let mut company = HashMap::new();

    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(error) => println!("Error reading from stdin: {}", error),
        }

        let mut res = match buffer.trim_end() {
            "" => break,
            input => input.split(" "),
        };

        if res.count() < 3 {
            break;
        }

        let person = res.nth(0);
        let department = res.nth(3);
        let department_persons = vec![person];

        let found_department_persons = company.entry(department).or_insert(department_persons);

        println!("{:?}", company);
        println!("{:?}", found_department_persons);
    }
}
