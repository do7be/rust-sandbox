fn main() {
    let img = image::open("./src/image/do7be.png").unwrap();
    println!("width: {}, height: {}", img.width(), img.height());
}
