use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

#[wasm_bindgen]
pub fn style_button_primary(element: Element) -> Result<(), JsValue> {
    let button = element.dyn_into::<HtmlElement>()?;

    button.set_class_name("px-4 py-2 bg-blue-500");
    button.set_class_name("text-white rounded-md");
    button.set_class_name("hover:bg-blue-700 transition");
    Ok(())
}

#[wasm_bindgen]
pub fn style_button_secondary(element: Element) -> Result<(), JsValue> {
    let button = element.dyn_into::<HtmlElement>()?;

    button.set_class_name("px-4 py-2 bg-gray-500");
    button.set_class_name("text-white rounded-md");
    button.set_class_name("hover:bg-gray-700 transition");

    Ok(())
}

