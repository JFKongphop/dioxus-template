use dioxus::prelude::*;

#[component]
pub fn TechStackDescriptionCard(description: String) -> Element {
  rsx! {
    p {
      class: "max-sm:text-[8px] text-xs opacity-50 text-center",
      {description}
    }
  }
}
