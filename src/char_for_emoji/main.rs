fn main() {
    let chars = "🦀👨‍👩‍👧‍👦🍅".chars().collect::<Vec<char>>();
    println!("{}", chars.len());
    println!("{}", chars[0]);
    println!("{:?}", chars);
}
