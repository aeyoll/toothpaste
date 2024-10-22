use crate::app::Route;
use yew::{classes, function_component, html, Children, Html, Properties};
use yew_router::components::Link;
use yew_router::hooks::use_route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub to: Route,
}

#[function_component(RouteLink)]
pub fn route_link(props: &Props) -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    let classes = if route == props.to {
        classes!(
            "rounded-md",
            "px-3",
            "py-2",
            "text-sm",
            "font-medium",
            "text-white",
            "bg-gray-900"
        )
    } else {
        classes!(
            "rounded-md",
            "px-3",
            "py-2",
            "text-sm",
            "font-medium",
            "text-white"
        )
    };

    html! {
        <Link<Route> classes={classes} to={props.to.clone()}>{for props.children.iter() }</Link<Route>>
    }
}
