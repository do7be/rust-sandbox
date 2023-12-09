#[derive(Debug)]
struct Hoge<T, U> {
    a: T,
    b: U,
}

impl<T, U> Hoge<T, U> {
    fn swap(self) -> Hoge<U, T> {
        Hoge {
            a: self.b,
            b: self.a,
        }
    }
}

fn main() {
    let hoge = Hoge { a: "aaaaaa", b: 2. };
    println!("{:?}", hoge);
    let swap = hoge.swap();
    println!("{:?}", swap);
}
