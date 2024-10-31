use yew::prelude::*;
use yew_router::prelude::*;

use yew::html::Scope;

use crate::components::route_link::RouteLink;
use crate::pages::get_paste::GetPaste;
use crate::pages::home::Home;
use crate::pages::new_paste::NewPaste;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/paste/:id")]
    GetPaste { id: String },
    #[at("/new")]
    NewPaste,
    #[at("/")]
    Home,
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
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="#ffffff" class="h-8 w-auto">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z" />
                                </svg>
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
        Route::Home => {
            html! { <Home /> }
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
