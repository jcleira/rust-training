fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    let x = 6;
    println!("The value of x is {}", x);

    {
        let x = 7;
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);

    let tup = (500, 6.4, '2');

    let (x, y, z) = tup;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is {}", a[0]);
}
