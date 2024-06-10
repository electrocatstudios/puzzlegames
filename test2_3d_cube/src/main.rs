use yew::prelude::*;
use yew_router::prelude::*;

mod game_control;
mod components;
mod utils;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/webgl_cube")]
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
