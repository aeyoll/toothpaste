use yew::prelude::*;
use yew_router::prelude::*;

use yew::html::Scope;

use crate::components::route_link::RouteLink;
use crate::pages::home::Home;
use crate::pages::new_paste::NewPaste;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/new")]
    NewPaste,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
}

pub struct App {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div>
                    { self.view_nav(ctx.link()) }

                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        html! {
            <nav class="bg-gray-800">
                <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                            <div class="flex flex-shrink-0 items-center">
                                <img class="h-8 w-auto" src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=500" alt="Paste" />
                            </div>
                            <div class="hidden sm:ml-6 sm:block">
                                <div class="flex space-x-4">
                                    <RouteLink to={Route::Home}>
                                        { "Home" }
                                    </RouteLink>

                                    <RouteLink to={Route::NewPaste}>
                                        { "New paste" }
                                    </RouteLink>

                                    <a href="https://github.com/aeyoll/toothpaste" target="_blank" rel="noopener" class="rounded-md px-3 py-2 text-sm font-medium text-white">{ "Github" }</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NewPaste => {
            html! { <NewPaste /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
