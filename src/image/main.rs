fn main() {
    let img = image::open("./src/image/do7be.png").unwrap();
    println!("width: {}, height: {}", img.width(), img.height());

    // チェック柄の画像を出力
    let mut img = image::RgbImage::new(200, 200);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let rgb = match x % 40 < 20 && y % 40 < 20 || x % 40 >= 20 && y % 40 >= 20 {
            true => [0, 0, 0],
            false => [255, 255, 255],
        };
        *pixel = image::Rgb(rgb);
    }
    img.save("./src/image/check.png").unwrap();
}
