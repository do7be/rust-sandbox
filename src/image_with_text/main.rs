use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let mut img = image::open("./src/image_with_text/background.png").unwrap();

    add_text_to_image(&mut img);

    img.save("./src/image_with_text/output.png").unwrap();
}

fn add_text_to_image(image: &mut DynamicImage) {
    let font_data = include_bytes!("./DelaGothicOne-Regular.ttf");
    let font = Font::try_from_bytes(font_data).expect("Failed to load font");

    let scale = Scale::uniform(24.0);
    let text_color = Rgba([255, 0, 0, 255]);

    let text = "This is a Tomato!";
    draw_text_mut(image, text_color, 20, 80, scale, &font, text);
}
