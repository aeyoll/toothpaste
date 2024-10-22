use yew::prelude::*;

pub struct NewPaste;

use crate::components::paste_form::PasteForm;

impl Component for NewPaste {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <h1 class="title">
                    { "New paste" }
                </h1>

                <PasteForm />
            </section>
        }
    }
}
