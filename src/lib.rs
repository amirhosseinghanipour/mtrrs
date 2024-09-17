mod components;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

// Entry point for the WebAssembly module
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // Find and style all primary buttons
    apply_rx_type_buttons(&document, "btn-primary", components::button::style_button_primary)?;

    // Find and style all secondary buttons
    apply_rx_type_buttons(&document, "btn-secondary", components::button::style_button_secondary)?;

    Ok(())
}

// Function to find and style buttons with a specific rx-type attribute
fn apply_rx_type_buttons(
    document: &Document,
    rx_type: &str,
    style_fn: fn(Element) -> Result<(), JsValue>,
) -> Result<(), JsValue> {
    let selector = format!("[rx-type='{}']", rx_type);
    let buttons = document.query_selector_all(&selector)?;

    for i in 0..buttons.length() {
        if let Some(button) = buttons.item(i) {
            style_fn(button)?;
        }
    }

    Ok(())
}
