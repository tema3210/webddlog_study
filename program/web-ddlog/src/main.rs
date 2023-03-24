#![recursion_limit = "1024"]
mod app;
mod store;
mod components;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}