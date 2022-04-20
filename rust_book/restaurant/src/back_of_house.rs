pub struct Breakfast {
    pub toast: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
        }
    }
}
