// TSで言うところの "Hello" | "World" みたいな型はenumでこう書ける
// ただしパスカルケースの文字列に限られる
#[derive(Debug)]
enum Hoge {
    Hello,
    World,
}

fn main() {
    let hello = hello_world(1);
    println!("{:?}", hello);

    let world = hello_world(-1);
    println!("{:?}", world);
}

fn hello_world(x: i32) -> Hoge {
    let result = if x > 0 { Hoge::Hello } else { Hoge::World };
    return result;
}
