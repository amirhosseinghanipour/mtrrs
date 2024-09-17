use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

#[wasm_bindgen]
pub fn style_button_primary(element: Element) -> Result<(), JsValue> {
    let button = element.dyn_into::<HtmlElement>()?;

    button.class_list().add_3("px-4", "py-2", "bg-blue-500")?;
    button.class_list().add_2("text-white", "rounded-md")?;
    button.class_list().add_2("hover:bg-blue-700", "transition")?;

    Ok(())
}

#[wasm_bindgen]
pub fn style_button_secondary(element: Element) -> Result<(), JsValue> {
    let button = element.dyn_into::<HtmlElement>()?;
    
    button.class_list().add_3("px-4", "py-2", "bg-gray-500")?;
    button.class_list().add_2("text-white", "rounded-md")?;
    button.class_list().add_2("hover:bg-gray-700", "transition")?;

    Ok(())
}

