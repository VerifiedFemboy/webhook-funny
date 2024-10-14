use std::{thread::sleep, time::Duration};
use ab_glyph::{FontArc, PxScale};
use glyph_brush::{GlyphBrushBuilder, GlyphCruncher, Section, Text};
use image::{Rgb, RgbImage};
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use imageproc::drawing::draw_text_mut;
use rand::Rng;
use serde_json::json;

use reqwest::Client;
use reqwest::multipart;

#[allow(dead_code)]
async fn send_webhook(client: &Client, url: &str, message: &str) {
    let payload = json!({
        "content": message
    });

    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            println!("Message sent successfully!");
        }
        Ok(response) => {
            eprintln!("Failed to send message: {:?}", response.status());
        }
        Err(err) => {
            eprintln!("Error sending webhook: {:?}", err);
        }
    }
}

async fn send_webhook_with_image(client: &Client, url: &str, image: RgbImage) {
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder.write_image(&image, image.width(), image.height(), image::ColorType::Rgb8.into()).unwrap();

    let part = multipart::Part::bytes(buf).file_name("image.png").mime_str("image/png").unwrap();
    let form = multipart::Form::new()
        .text("payload_json", json!({"content": ""}).to_string())
        .part("file", part);

    let res = client
        .post(url)
        .multipart(form)
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            println!("Message sent successfully!");
        }
        Ok(response) => {
            eprintln!("Failed to send message: {:?}", response.status());
        }
        Err(err) => {
            eprintln!("Error sending webhook: {:?}", err);
        }
    }
}
    

async fn random_meesage() -> String {
    let mut rng = rand::thread_rng();
    let chars: String = (0..2)
        .map(|_| {
            let idx = rng.gen_range(0..26);
            let c = (b'a' + idx as u8) as char;
            c
        })
        .collect();
    let first_char = (b'A' + rng.gen_range(0..26) as u8) as char;
    format!("{}{}", first_char, chars)
}

async fn gen_image(text: &str) -> RgbImage {
    let font_data = include_bytes!("../comicsans.ttf");
    let font = FontArc::try_from_slice(font_data).unwrap();

    let height: i32 = 400;
    let width: u32 = 800;
    
    let mut image = RgbImage::new(width, height as u32);

    let scale = PxScale::from(120.0);

    let text_color = Rgb([255, 255, 255]);
    let mut glyph_brush: glyph_brush::GlyphBrush<ab_glyph::FontArc> = GlyphBrushBuilder::using_font(font.clone()).build();
    let section = Section::default().add_text(Text::new(text).with_scale(scale));
    let text_width = glyph_brush.glyph_bounds(section).map_or(0.0, |bounds| bounds.width()) as u32;
    let x = (width - text_width) / 2;
    let y = (height - scale.y as i32) / 2;
    draw_text_mut(&mut image, text_color, x.try_into().unwrap(), y, scale, &font, text);

    image
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let webhook_url = "https://discord.com/api/webhooks/your_webhook_url_here";

    loop {
        let message = random_meesage().await;
        let image = gen_image(&message).await;
        // send_webhook(&client, webhook_url, &message).await;
        send_webhook_with_image(&client, webhook_url, image).await;
        sleep(Duration::from_secs(600));
    }
}
