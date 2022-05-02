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
}
