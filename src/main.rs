use dioxus::prelude::*;
// use crate::CardComponent;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            h1 {id: "Title", "Welcome, to Visualnary" }
        }
        // section { class: "navigation",

        //     div {
        //         h2 { "About" }
        //         p { "Visualnary is a platform for keeping track of " }
        //     }
        //     div {
        //         h2 { "Features" }
        //         ul {
        //             li { "Create and share visual content" }
        //             li { "Collaborate with others" }
        //             li { "Discover new content" }
        //         }
        //     }
        // }
        section { id: "Body",
            div { class: "search-container",
                input { type: "text", placeholder: "Search..." }
                button { "Search" }
            }
            // div { class: "cards",
            //     for i in 0..4 {
            //         CardComponent { title_card: &format!("Card {}", i + 1), content_card: "This is a card content" }
            //     }
            // }
        }
    }
}

// #[component]
// pub fn CardComponent<'a>(title_card: &'a str, content_card: &'a str) -> Element {
//     rsx! {
//         div { class: "card",
//             h3 { class: "card-title", "{title_card}" }
//             p { class: "card-content", "{content_card}" }
//         }
//     }
// }
