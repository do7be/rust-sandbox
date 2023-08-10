use colored::*;

// こっちのほうが変数宣言はきれい
struct RGB1(u8, u8, u8);

// こっちのほうが引数渡すときはよさげ
struct RGB2 {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let green = RGB1(0, 255, 0);
    let yellow = RGB2 {
        r: 255,
        g: 255,
        b: 0,
    };

    println!("{} in red", "■".red());
    println!("{} in green", "■".truecolor(green.0, green.1, green.2));
    println!("{} in yellow", "■".truecolor(yellow.r, yellow.g, yellow.b));
}
