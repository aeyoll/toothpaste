mod app;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
