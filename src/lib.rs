use rand::Rng;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Circle {
    x: i32,
    y: i32,
    radius: u32,
}

impl Circle {
    fn new(x: i32, y: i32, radius: u32) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    pub fn kirby_circles(
        height: i32,
        width: i32,
        regions: u32,
        density: u32,
        spread: i32,
    ) -> Vec<Circle> {
        let mut rng = rand::thread_rng();

        let mut circles: Vec<Circle> = vec![];

        for _ in 0..regions {
            let mut x: i32 = rng.gen_range(0..width);
            let mut y: i32 = rng.gen_range(0..height);

            for _ in 0..density {
                x += rng.gen_range((0 - spread)..spread);
                y += rng.gen_range((0 - spread)..spread);
                let radius: u32 = rng.gen_range(2..(spread as u32));

                circles.push(Circle::new(x, y, radius));
            }
        }
        return circles;
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn generate_canvas(
    color1: String,
    color2: String,
    regions: u32,
    density: u32,
    spread: i32,
) -> () {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let canvas_div = document
        .get_element_by_id("canvasdiv")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let height = canvas_div.client_height();
    let width = canvas_div.client_width();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, width as f64, height as f64);
    context.set_fill_style(&color1.clone().into());
    context.fill_rect(0.0, 0.0, width as f64, height as f64);

    let circles = Circle::kirby_circles(height as i32, width as i32, regions, density, spread);
    context.set_fill_style(&color2.clone().into());
    for c in &circles {
        context.begin_path();
        context
            .arc(
                c.x as f64,
                c.y as f64,
                c.radius as f64,
                0.0,
                2.0 * f64::consts::PI,
            )
            .unwrap();
        context.fill();
    }
}
