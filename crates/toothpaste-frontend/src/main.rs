mod app;
mod components;
mod pages;

use app::App;

fn main() {
    // If feature "log" is enabled, logs will be printed to the browser console
    #[cfg(feature = "log")] {
        wasm_logger::init(wasm_logger::Config::default());
        log::debug!("Logging enabled");
    }

    yew::Renderer::<App>::new().render();
}
