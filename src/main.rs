use dioxus::prelude::*;

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
            div { class: "cards",
            div { class: "card",
                h3 { class: "card-title", "Card 1" }
                p { class: "card-content", "This is a card" }
            }
            div { class: "card",
                h3 { class: "card-title", "Card 2" }
                p { class: "card-content", "This is a card" }
            }
            div { class: "card",
                h3 { class: "card-title", "Card 3" }
                p { class: "card-content", "This is a card" }
            }
            }
        }
    }
}
