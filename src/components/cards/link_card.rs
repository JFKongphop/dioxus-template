use dioxus::prelude::*;

#[component]
pub fn LinkCard(icon: Element, link: String, name: String) -> Element {
  rsx! {
    div { 
      class: "hover:ring-2 hover:ring-[#4b5563] px-4 py-2 rounded-4xl w-full flex flex-row items-center gap-2 bg-[#111827] cursor-pointer justify-center",
      {icon}
      a {  
        href: link,
        target: "_blank",
        class: "text-white text-sm tracking-wider",
        {name}
      }
    }
  }
}