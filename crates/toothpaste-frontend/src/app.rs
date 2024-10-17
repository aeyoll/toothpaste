use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
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
            <div>
                { self.view_nav(ctx.link()) }
                <main>
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </main>
            </div>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark mb-3">
                <div class="container-fluid">
                    <a class="navbar-brand" href="/">{ "Paste" }</a>

                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent"
                        aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>

                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                            <li class="nav-item">
                                <a class="nav-link" href="/paste/create">{ "New paste" }</a>
                            </li>
                        </ul>

                        <div class="d-flex">
                            <a href="https://github.com/aeyoll/toothpaste" target="_blank" rel="noopener">
                                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor"
                                    class="bi bi-github" viewBox="0 0 16 16">
                                    <path
                                        d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z" />
                                </svg>
                                <span class="visually-hidden">{ "github" }</span>
                            </a>
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
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
