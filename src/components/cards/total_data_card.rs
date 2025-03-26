use dioxus::prelude::*;

#[component]
pub fn TotaDatalCard(window_size: String, data: String, unit: String) -> Element {
  #[allow(unused_assignments)]
  let mut element: Element = rsx!{};
  
  if window_size == "sm" {
    element = rsx!{
      p {
        class: "text-lg", 
        "{data}" 
      }
      p {  
        class: "text-xs",
        "{unit}"
      }
    }
  } else {
    element = rsx!{
      p {
        class: "text-xl", 
        "{data} {unit}" 
      }
    }
  }

  rsx! {
    div { 
      class: "flex max-sm:flex-col justify-center items-center w-full",
      {element}
    }
  }
}
