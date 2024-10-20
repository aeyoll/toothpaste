use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="py-12">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold text-gray-900 mb-4">
                            { "Page not found" }
                        </h1>
                        <h2 class="text-xl text-gray-600">
                            { "This page does not seem to exist" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
