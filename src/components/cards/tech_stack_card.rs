use dioxus::prelude::*;

#[component]
pub fn TechStackCard(element: Element) -> Element {
  rsx! {
    div {
      class: "border h-20 border-bright-shade rounded-lg hover:border-white flex flex-col justify-center items-center p-2 gap-1",
      {element}
    }
  }
}
