use dioxus::prelude::*;

// #[inline_props]
// pub fn NotFound(cx: Scope,segments: Vec<String> ) -> Element {
//     render!(rsx!(
//         div{
//             "not found"
//         }
//     ))
// }


#[inline_props]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
