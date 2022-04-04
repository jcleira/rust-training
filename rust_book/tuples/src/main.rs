struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "Hello, {}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count,
    );

    let user2 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        ..user1
    };

    println!(
        "Hello, {}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count,
    );

    println!(
        "Hello, {}, {}, {}, {}",
        user2.email, user2.username, user2.active, user2.sign_in_count,
    );

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect3)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    dbg!(&rect3);

    println!("rect3 is {:#?}", rect3);
}

fn area(with: u32, height: u32) -> u32 {
    with * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
