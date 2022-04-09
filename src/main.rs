mod calculate;
pub mod config;
mod home;
mod inputs;
mod options;
mod utils;

use home::Home;
use yew::prelude::*;

use calculate::calculate;
use options::{Options, OptionsName};

#[function_component(App)]
fn app() -> Html {
    html! {
      <Home />
    }
}

fn main() {
    yew::start_app::<App>();
}
