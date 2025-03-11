#![allow(non_snake_case)]
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

const JFK_KONGPHOP: Asset = asset!("/assets/JFKongphop.jpg");
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin, FaMedium};
use dioxus_free_icons::Icon;
use dioxus_vercel::components::cards::link_card::LinkCard;
use dioxus_vercel::components::cards::tech_stack_card::TechStackCard;
use dioxus_vercel::components::cards::tech_stack_description::TechStackDescriptionCard;
use dioxus_vercel::utils::github_data::github_contribution;
use dioxus_vercel::constants::tech_stack_data::TECH_STACK;

fn main() {
  launch(App);
}

fn App() -> Element {
  let contribution = use_resource(move || github_contribution());

  match &*contribution.read() {
    Some(Ok(count)) => info!("hello world {}", count),
    Some(Err(err)) => println!("Error fetching contributions: {}", err),
    None => println!("Loading contributions..."),
  }

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
      div {
        class: "second-screen w-full min-h-screen flex flex-col gap-4 justify-center items-center p-8",
        div {
          class: "w-full h-full flex flex-col gap-8 items-center text-lg max-sm:text-xs justify-center",
          div {
            class: "w-2/3 max-sm:w-full flex flex-col gap-6",
            match &*contribution.read() {
              Some(Ok(count)) => rsx! {
                p {
                  class: "text-center text-2xl",
                  "{count} contributions last year"
                }
                img {
                  src: "https://ghchart.rshah.org/4b5563/JFKongphop",
                  class: "w-full"
                }
              },
              Some(Err(err)) => rsx! {
                div {
                  "Error fetching contributions: {err}"
                }
              },
              None => rsx! {
                p {
                  class: "text-center text-2xl",
                  "Loading contribution..."
                }
              },
            }
          }
          div {
            class: "w-2/3 max-sm:w-full flex flex-col gap-6",
            p {  
              class: "text-center text-2xl",
              {"Tech Stacks"}
            }
            div {  
              class: "grid grid-cols-4 max-sm:grid-cols-2 col-span-1 gap-2",
              for tech_stack in &*TECH_STACK {
                TechStackCard { 
                  element: rsx!{
                    p {
                      {tech_stack.key.clone()}
                    }
                    TechStackDescriptionCard {
                      description: tech_stack.description.clone()
                    }
                  },
                  key: tech_stack.key
                }
              }
            }
          }
        }
      }
    }
  }
}
