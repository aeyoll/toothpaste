mod app;
mod pages;
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
