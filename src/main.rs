#![allow(non_snake_case)]
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use web_sys::window;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin, FaMedium};
use dioxus_free_icons::icons::fa_solid_icons::FaLink;
use dioxus_free_icons::Icon;

use dioxus_vercel::components::cards::link_card::LinkCard;
use dioxus_vercel::components::cards::tech_stack_card::TechStackCard;
use dioxus_vercel::components::cards::tech_stack_description::TechStackDescriptionCard;
use dioxus_vercel::constants::tech_stack_data::TECH_STACK;
use dioxus_vercel::utils::github_data::github_contribution;

const JFK_KONGPHOP: Asset = asset!("/assets/JFKongphop.jpg");
const DIOXUS: Asset = asset!("/assets/dioxus.png");
const ZK_DEBIT: Asset = asset!("/assets/zkDebit.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  launch(App);
}

fn App() -> Element {
  let contribution = use_resource(move || github_contribution());

  let mut width = use_signal(|| 0);
  let mut height = use_signal(|| 0);

  use_effect(move || {
    let w = window()
      .expect("There should be a window")
      .inner_width()
      .expect("The window should have Some width")
      .as_f64()
      .expect("The width should be a number") as i32;

    let h = window()
      .expect("There should be a window")
      .inner_height()
      .expect("The window should have Some width")
      .as_f64()
      .expect("The width should be a number") as i32;

    width.set(w - 64);
    height.set(h - 64);

    let c = window().expect("a").document().expect("b").get_element_by_id("1").expect("c").client_height();//.as_f64().expect("d");

    // window().expect("a").event().cl

    info!(c);


  });


  rsx! {
    document::Stylesheet {
      href: TAILWIND_CSS
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
              "Tech Stacks"
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
          div { 
            class: "w-2/3 max-sm:w-full flex flex-col gap-6",
            p {  
              class: "text-center text-2xl",
              "Hackathons"
            }
            div {  
              class: "flex flex-col",
              div {  
                class: "w-full flex flex-row max-sm:flex-col gap-4",
                div {  
                  class: "w-3/5 max-sm:w-full flex flex-col justify-between gap-2",
                  div {  
                    class: "w-full flex flex-col gap-2",
                    p {  
                      class: "text-xl",
                      "zkDebit"
                    }
                    p {  
                      class: "text-base opacity-50",
                      "zkDebit is an innovative payment platform that leverages zero-knowledge proofs (ZKPs) to enhance privacy and security in digital transactions. Unlike traditional payment systems, zkDebit eliminates the need to share sensitive details such as card numbers, CVVs, or expiration dates with merchants. Instead, users generate and submit a cryptographic proof verifying their ownership of the card and transaction validity."
                    }
                  }
                  div {  
                    class: "flex flex-row justify-between items-center",
                    p {
                      class: "text-base",
                      "(Backend and Smart Contract)"
                    }
                    a {
                      href: "https://ethglobal.com/showcase/zkdebit-ypjir",
                      target: "_blank",
                      Icon {
                        width: 16,
                        height: 16,
                        fill: "white",
                        icon: FaLink,
                      }
                    }
                  }
                }
                div {
                  class: "w-2/5 max-sm:w-full h-[300px]",
                  img {
                    src: ZK_DEBIT,
                    class: "rounded-2xl h-full w-full"
                  }
                }
              }
            }
          }
        }
      }
      div {  id: "1",  "aaaǎ"}
      div {
        class: "w-full h-[40px]"
      }
      div {
        class: "slider-container",
        div {
          class: "slider",
          div {
            class: "slide",
            p {"POWERED BY"}
            p {"DIOXUS"}
            img {
              src: DIOXUS,
              class: "w-8 h-8"
            }
          }
          div {
            class: "slide",
            p {"POWERED BY"}
            p {"DIOXUS"}
            img {
              src: DIOXUS,
              class: "w-8 h-8"
            }
          }
        }
      }
    }
  }
}
