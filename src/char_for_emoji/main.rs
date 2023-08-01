fn main() {
    let original = "🦀👨‍👩‍👧‍👦🍅";
    let chars = original.chars().collect::<Vec<char>>();
    println!("original: {}", original);
    println!("length: {}", chars.len());
    println!("first: {}", chars[0]);
    println!("list: {:?}", chars);

    let filtered: String = chars
        .clone()
        .iter()
        .filter(|&&char| char != '\u{200d}')
        .collect();

    println!("disassembled: {}", filtered);
}
