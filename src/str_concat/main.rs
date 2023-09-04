fn main() {
    // Stringに&strを連結するケース

    // String + &str
    let a = "aaa".to_string();
    let b = "bbb";
    let c = a + b;
    println!("{}", c);

    // String.push_str(&str)
    let mut a = "aaa".to_string();
    let b = "bbb";
    a.push_str(b);
    println!("{}", a);

    // format!(String, &str)
    let a = "aaa";
    let b = "bbb";
    let c = format!("{}{}", a, b);
    println!("{}", c);

    // StringにStringを連結するケース

    // String.push_str(&String)
    let mut a = "aaa".to_string();
    let b = "bbb".to_string();
    a.push_str(&b);
    println!("{}", a);

    // format!(String, String)
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let c = format!("{}{}", a, b);
    println!("{}", c);
}
