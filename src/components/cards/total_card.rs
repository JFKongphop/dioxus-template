use dioxus::prelude::*;

#[component]
pub fn TotalCard(window_size: String, element: Element) -> Element {
  rsx! {
    p {
      class: format!(
        "flex flex-row max-sm:gap-2 justify-between w-full {}:hidden border-b pb-2", 
        window_size
      ),
      {element}
    }
  }
}
