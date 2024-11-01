use yew::prelude::*;
use yew_router::prelude::*;

use yew::html::Scope;

use crate::components::route_link::RouteLink;
use crate::pages::about::About;
use crate::pages::get_paste::GetPaste;
use crate::pages::new_paste::NewPaste;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/paste/:id")]
    GetPaste { id: String },
    #[at("/about")]
    About,
    #[at("/")]
    NewPaste,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
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
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="bg-gray-800 mb-3">
                <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                            <div class="flex flex-shrink-0 items-center">
                                <img src="static/logo.svg" alt="Toothpaste logo" width="24" height="24" />
                            </div>
                            <div class="hidden sm:ml-6 sm:block">
                                <div class="flex space-x-4">
                                    <RouteLink to={Route::NewPaste}>
                                        <div class="flex items-center text-white gap-2">
                                            <img src="static/plus.svg" alt="New paste" width="18" height="18" />
                                            { "New paste" }
                                        </div>
                                    </RouteLink>

                                    <RouteLink to={Route::About}>
                                        <div class="flex items-center text-white gap-2">
                                            <img src="static/info.svg" alt="About" width="18" height="18" />
                                            { "About" }
                                        </div>
                                    </RouteLink>

                                    <a href="https://github.com/aeyoll/toothpaste" target="_blank" rel="noopener" class="rounded-md px-3 py-2 text-sm font-medium text-white">
                                        <div class="flex items-center text-white gap-2">
                                            <img src="static/github.svg" alt="Github" width="18" height="18" />
                                            { "Github" }
                                        </div>
                                    </a>

                                    <span class="rounded-md px-3 py-2 text-sm font-medium text-gray-300">
                                        { format!("v{}", env!("CARGO_PKG_VERSION")) }
                                    </span>
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
        Route::About => {
            html! { <About /> }
        }
        Route::GetPaste { id } => {
            html! { <GetPaste id={id} /> }
        }
        Route::NewPaste => {
            html! { <NewPaste /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
