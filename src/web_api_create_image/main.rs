use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    text: String,
}

#[get("/hello")]
async fn hello_tomato() -> HttpResponse {
    HttpResponse::Ok().body("Hello Tomato!")
}

#[get("/create")]
async fn create(query: web::Query<Body>) -> impl Responder {
    let mut img = image::open("./src/web_api_create_image/background.png").unwrap();

    add_text_to_image(&mut img, &query.text);

    let mut buffer = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut buffer),
        image::ImageOutputFormat::Jpeg(80),
    )
    .expect("Failed to encode image");

    HttpResponse::Ok().content_type("image/png").body(buffer)
}

fn add_text_to_image(image: &mut DynamicImage, text: &str) {
    let font_data = include_bytes!("./DelaGothicOne-Regular.ttf");
    let font = Font::try_from_bytes(font_data).expect("Failed to load font");

    let scale = Scale::uniform(24.0);
    let text_color = Rgba([255, 0, 0, 255]);

    draw_text_mut(image, text_color, 20, 80, scale, &font, text);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/hoge", "./src/web_api_create_image/static"))
            .service(hello_tomato)
            .service(create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
