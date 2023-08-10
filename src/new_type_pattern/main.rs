use std::fmt;

struct UpperCase(String);

impl fmt::Display for UpperCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_uppercase())
    }
}

fn main() {
    let hoge = UpperCase(String::from("essential gati kusa tomato"));
    println!("{}", hoge);
}
