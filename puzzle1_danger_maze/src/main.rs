use yew::prelude::*;
use yew_router::prelude::*;

mod game_control;
mod game_components;
mod level_builder;
mod utils;
mod levels;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/danger_maze")]
    Home,
    #[at("/level_builder")]
    LevelBuilder,
}

fn switch(routes: Route) -> Html {    
    match routes {
        Route::Home => html!{
            <game_control::GameControl />
        },
        Route::LevelBuilder => html! {
            <level_builder::LevelBuilder />
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
