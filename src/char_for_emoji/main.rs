fn main() {
    // terminalだと家族絵文字が最初から分解されてしまう
    let original = "🦀👨‍👩‍👧‍👦🍅";
    let chars = original.chars().collect::<Vec<char>>();
    println!("original: {}", original);
    println!("length: {}", chars.len());
    println!("first: {}", chars[0]);
    println!("list: {:?}", chars);

    let disassembled: String = chars
        .clone()
        .iter()
        .filter(|&&char| char != '\u{200d}') // char生成はシングルクォート
        .collect();

    // 1人1人バラバラになった家族
    println!("disassembled: {}", disassembled);
}
