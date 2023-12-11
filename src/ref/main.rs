fn main() {
    let hoge = Some(String::from("aaaaaa"));
    match hoge {
        // refをつけることでmoveせず借用する
        Some(ref fuga) => println!("{}", fuga),
        None => println!("None"),
    }
    println!("hoge is {:?}", hoge);

    let mut hoge2 = Some(String::from("aaaaaa"));
    match hoge2 {
        // ref mutをつけることで可変参照になる
        Some(ref mut fuga) => *fuga = String::from("piyo"),
        None => println!("None"),
    }
    println!("hoge2 is {:?}", hoge2);
}
