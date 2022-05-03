fn main() {
    let s = String::new();
    println!("{}", s);

    let data = "initial content";
    let s_data = data.to_string();

    println!("{}", s_data);

    let s = String::from("initial content with sense");
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
