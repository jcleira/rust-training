use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Red"), String::from("Green")];
    let scores = vec![10, 25];
    let team_scores: HashMap<_, _> = teams.into_iter().zip(scores).collect();

    println!("{:?}", team_scores);
}
