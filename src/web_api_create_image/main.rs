use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    text: String,
}

#[get("/")]
async fn hello_tomato() -> impl Responder {
    HttpResponse::Ok().body("Hello Tomato!")
}

#[post("/create")]
async fn create(req_body: web::Json<Body>) -> impl Responder {
    let mut img = image::open("./src/web_api_create_image/background.png").unwrap();

    add_text_to_image(&mut img, &req_body.text);

    img.save("./src/web_api_create_image/output.png").unwrap();

    HttpResponse::Ok().body("Created!")
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
    HttpServer::new(|| App::new().service(hello_tomato).service(create))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
