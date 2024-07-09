use yew::prelude::*;
use yew_router::prelude::*;

use gloo_console::log;

mod game_control;
mod mouse_handler;
mod utils;
mod rope_manager;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {    
    match routes {
        Route::Home => html!{
            <game_control::GameControl />
        }
    }
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
