mod app;
mod components;
mod pages;
mod route;
mod utils;

use app::App;
use bounce::BounceRoot;
use yew::prelude::*;

#[function_component(AppWrapper)]
fn app_wrapper() -> Html {
    html! {
        <BounceRoot>
            <App />
        </BounceRoot>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<AppWrapper>::new().render();
}
