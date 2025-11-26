/*use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reverse(word: &str) -> String {
    let rewerse_word: String = word.chars().rev().collect();
    rewerse_word

}*/

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get window and document
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("should have a document on window")?;

    // Find canvas by id
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("canvas element not found")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Get 2D context
    let ctx = canvas
        .get_context("2d")?
        .ok_or("failed to get 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Draw a line from (20, 20) to (200, 120)
    ctx.begin_path();
    ctx.move_to(20.0, 20.0);
    ctx.line_to(200.0, 120.0);
    ctx.set_stroke_style_str("#b02b32ff");
    //ctx.set_stroke_style(&JsValue::from_str("#2b6cb0")); // blue-ish
    ctx.set_line_width(4.0);
    ctx.stroke();

    Ok(())
}
