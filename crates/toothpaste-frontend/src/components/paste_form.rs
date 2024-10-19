use indexmap::indexmap;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PasteForm)]
pub fn paste_form(_props: &Props) -> Html {
    // Expire_after key/value hashmap
    let expire_after_values = indexmap! {
        "1800" => "30 minutes",
        "21600" => "6 hours",
        "86400" => "1 day",
        "604800" => "1 week",
        "2592000" => "1 month",
        "0" => "forever",
    };
    let expire_after_options = expire_after_values
        .iter()
        .map(|(key, value)| {
            let selected = *key == "86400";

            html! {
                <option value={key.to_string()} selected={selected}>{value}</option>
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <form>
                <div class="mb-3">
                    <label for="filename" class="form-label">{"Filename"}</label>
                    <input type="text" name="filename" id="filename" class="form-control" required=true />
                    <div class="form-help">{"Pro-tip: use a file extension to get syntax coloration!"}</div>
                </div>

                <div class="mb-3">
                    <label for="content" class="form-label">{"Content"}</label>
                    <textarea name="content" id="content" class="form-control" required=true rows="20"></textarea>
                </div>

                <div class="mb-3">
                    <label for="expire_after" class="form-label">{"Expire"}</label>
                    <select id="expire_after" name="expire_after" class="form-control">
                        { expire_after_options }
                    </select>
                </div>

                <div class="mb-3 form-check">
                    <input type="checkbox" name="private" id="private" class="form-check-input" value="true" checked=true />
                    <label for="private" class="form-check-label">{"Private"}</label>
                    <div class="form-help">{"When private, the paste does not appear in the homepage"}</div>
                </div>

                <div class="mb-3">
                    <button type="submit" class="btn btn-primary">{"Create"}</button>
                </div>
            </form>
        </div>
    }
}
