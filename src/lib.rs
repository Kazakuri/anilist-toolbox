#![recursion_limit = "8092"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod agents;
pub mod app;
pub mod components;
pub mod routes;
pub mod utils;

use wasm_bindgen::prelude::*;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
  Ok(())
}
