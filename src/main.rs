#![allow(non_snake_case)]
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/JFKongphop.jpg");

fn main() {
  launch(App);
}

fn App() -> Element {
  let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "item 8"];

  rsx! {
    document::Stylesheet {
      href: asset!("/assets/tailwind.css")
    }

    div {
      class: "first-screen",
      img { src: HEADER_SVG, class: "h-10 w-10" }

      p {class: "bg-bright-shade", "hello world"}

      ul {

        for item in items {

            li { class: "text-yellow-500", key: item, "{item}" }

        }

    }
    }
  }
}
