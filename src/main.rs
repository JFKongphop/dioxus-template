#![allow(non_snake_case)]
use dioxus::prelude::*;

const JFK_KONGPHOP: Asset = asset!("/assets/JFKongphop.jpg");
use dioxus_free_icons::icons::fa_brands_icons::{FaMedium, FaLinkedin, FaGithub};
use dioxus_free_icons::Icon;
use dioxus_vercel::components::cards::link_card::LinkCard;

fn main() {
  launch(App);
}

fn App() -> Element {


  rsx! {
    document::Stylesheet {
      href: asset!("/assets/tailwind.css")
    }
    head {
      link {
          rel: "preconnect",
          href: "https://fonts.googleapis.com"
      }
      link {
          rel: "preconnect",
          href: "https://fonts.gstatic.com",
          crossorigin: "anonymous"
      }
      link {
          href: "https://fonts.googleapis.com/css2?family=Fjalla+One&family=Sixtyfour+Convergence&display=swap",
          rel: "stylesheet"
      }
    }

    div {
      class: "min-h-screen",
      div {  
        class: "first-screen w-full h-screen flex flex-col gap-4 justify-center items-center p-8",
        div {
          class: "w-full flex sm:justify-center",
          div {
            class: "flex flex-row items-center gap-2",
            img { 
              src: JFK_KONGPHOP,
              class: "w-8 h-8 rounded-full"
            } 
            p { 
              class: "text-xl", 
              "JFKongphop" 
            }
          }
        }
        div {
          class: "w-full flex sm:justify-center gap-2",
          div {
            class: "flex flex-row gap-2",
            LinkCard {
              icon: rsx!{Icon {
                width: 16,
                height: 16,
                fill: "white",
                icon: FaGithub,
              }},
              link: "https://github.com/JFKongphop".to_string(),
              name: "Github".to_string(),
            },
            LinkCard {
              icon: rsx!{Icon {
                width: 16,
                height: 16,
                fill: "white",
                icon: FaLinkedin,
              }},
              link: "https://www.linkedin.com/in/kongphop-kingpeth-225308236".to_string(),
              name: "Linkedin".to_string(),
            },
            LinkCard {
              icon: rsx!{Icon {
                width: 16,
                height: 16,
                fill: "white",
                icon: FaMedium,
              }},
              link: "https://medium.com/@kongphopkingpethp".to_string(),
              name: "Medium".to_string(),
            },
          }
        }
        div { 
          class: "flex flex-col gap-4 w-1/2 text-center max-sm:w-full max-sm:text-start max-sm:text-3xl",
          p {  
            class: "text-5xl leading-14 max-sm:text-3xl max-sm:leading-10",
            "Enthusiastic about"
            br {  }
            "Blockchain Application,"
            br {  }
            "Full-Stack, and Distance Running"
          }
          p { 
            class: "text-lg opacity-45",
            "I’m pursuing studies in Financial Engineering but have a strong interest in Computer Science, particularly in Blockchain technology and Full-Stack development."
            br { } 
            "I’m also passionate about track distance running."
          }
        }
      }
    }
  }
}


// let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "item 8", "item 9"];

// img { src: JFK_KONGPHOP, class: "h-10 w-10" }

// p {class: "bg-bright-shade", "hello world"}

// ul {

//   for item in items {

//       li { class: "text-green-500", key: item, "{item}" }

//   }

// }