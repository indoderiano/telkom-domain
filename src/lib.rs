mod app;
mod pages;
pub mod components;
// mod store;
// mod types;
mod configs;

use yewdux::prelude::WithDispatch;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<WithDispatch<app::App>>();

    Ok(())
}
