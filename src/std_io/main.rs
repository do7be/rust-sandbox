use rand::Rng;
use std::io;

fn main() {
    let random_1 = rand::thread_rng().gen_range(1..10);
    let random_2 = rand::thread_rng().gen_range(1..10);
    let answer = random_1 + random_2;

    println!("{} + {} = ?", random_1, random_2);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed");

    let input: u32 = input.trim().parse().expect("Not number");

    if input == answer {
        println!("正解");
    } else {
        println!("不正解");
    }
}
