mod ir;
mod runtime;
mod screen;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use ir::Program;
use runtime::Runtime;

#[derive(Debug, Serialize)]
struct PreviewData {
    top_text: Vec<String>,
    bottom_text: Vec<String>,

    top_pixels: Vec<PixelData>,
    bottom_pixels: Vec<PixelData>,

    top_rectangles: Vec<RectData>,
    bottom_rectangles: Vec<RectData>,

    top_background: u32,
    bottom_background: u32,
}

#[derive(Debug, Serialize)]
struct PixelData {
    x: usize,
    y: usize,
    color: u32,
}

#[derive(Debug, Serialize)]
struct RectData {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
}

#[wasm_bindgen]
pub fn test() -> String {
    "HBView3 Rust WASM fonctionne".to_string()
}

#[wasm_bindgen]
pub fn render(program: JsValue) -> Result<JsValue, JsValue> {
    let program: Program =
        serde_wasm_bindgen::from_value(program)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let mut runtime = Runtime::new();

    runtime.execute(&program);

    let top = runtime.top_screen();
    let bottom = runtime.bottom_screen();

    let preview = PreviewData {
        top_text: top
            .text
            .iter()
            .map(|line| line.text.clone())
            .collect(),

        bottom_text: bottom
            .text
            .iter()
            .map(|line| line.text.clone())
            .collect(),

        top_pixels: top
            .pixels
            .iter()
            .map(|pixel| PixelData {
                x: pixel.x,
                y: pixel.y,
                color: pixel.color,
            })
            .collect(),

        bottom_pixels: bottom
            .pixels
            .iter()
            .map(|pixel| PixelData {
                x: pixel.x,
                y: pixel.y,
                color: pixel.color,
            })
            .collect(),

        top_rectangles: top
            .rectangles
            .iter()
            .map(|rect| RectData {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                color: rect.color,
            })
            .collect(),

        bottom_rectangles: bottom
            .rectangles
            .iter()
            .map(|rect| RectData {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                color: rect.color,
            })
            .collect(),

        top_background: top.background_color,
        bottom_background: bottom.background_color,
    };

    serde_wasm_bindgen::to_value(&preview)
        .map_err(|error| JsValue::from_str(&error.to_string()))
}