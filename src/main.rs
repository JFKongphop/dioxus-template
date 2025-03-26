#![allow(non_snake_case)]
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin, FaMedium};
use dioxus_free_icons::icons::fa_solid_icons::FaLink;
use dioxus_free_icons::Icon;

use dioxus_vercel::components::cards::link_card::LinkCard;
use dioxus_vercel::components::cards::tech_stack_card::TechStackCard;
use dioxus_vercel::components::cards::tech_stack_description::TechStackDescriptionCard;
use dioxus_vercel::components::cards::total_card::TotalCard;
use dioxus_vercel::components::cards::total_data_card::TotaDatalCard;
use dioxus_vercel::constants::tech_stack_data::TECH_STACK;
use dioxus_vercel::types::running_response_types::TotalResponse;
use dioxus_vercel::utils::chart_percentage::apply_bar_percentage;
use dioxus_vercel::utils::fetch_api::{get_month_daily_distance, get_total_distance};
use dioxus_vercel::utils::window_data::WindowData;
use dioxus_vercel::utils::github_data::github_contribution;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

const JFK_KONGPHOP: Asset = asset!("/assets/JFKongphop.jpg");
const DIOXUS: Asset = asset!("/assets/dioxus.png");
const ZK_DEBIT: Asset = asset!("/assets/zkDebit.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  launch(App);
}

fn App() -> Element {
  let contribution = use_resource(move || github_contribution());
  let month_daily_distance_unread = use_resource(move || get_month_daily_distance());
  let total_distance_unread = use_resource(move || get_total_distance());

  let month_daily_distance = &*month_daily_distance_unread.read();
  let total_distance = &*total_distance_unread.read();

  let month_daily_percentage = match month_daily_distance {
    Some(Ok(distance)) => apply_bar_percentage(distance.clone()),
    Some(Err(_)) => vec![], 
    None => vec![],
  };
  info!("{:#?}", month_daily_percentage);

  let total_distance = match total_distance {
    Some(Ok(total)) => total.clone(),
    Some(Err(_)) => TotalResponse { distance: 0.0, running_activity: 0, running_day: 0 },
    None => TotalResponse { distance: 0.0, running_activity: 0, running_day: 0 },
  };
  
  let mut window_size = use_signal(|| (0, 0));
  let mut element_size = use_signal(|| (0, 0));
  let mut gl = use_signal(|| 0);

  
  use_effect(move || {
    let window_data = WindowData::new();
    let screen_size = window_data.screen_size();
    let div_size = window_data.element_id_size("first-graph");
    
    window_size.set(screen_size);
    element_size.set(div_size);
  });
  
  let month_days = 30;
  use_effect(move || {
    let first_index = element_size.read().0;
    let graph_lenght = (first_index - (month_days * 16)) / month_days;

    gl.set(graph_lenght);
  });
  
  use_effect(move || {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let graph = document.get_element_by_id("first-graph").unwrap();

    let closure = Closure::wrap(Box::new(move 
    |event: web_sys::MouseEvent| {
      info!("{:#?}", event.offset_x());
    }) as Box<dyn FnMut(_)>);

    graph
      .add_event_listener_with_callback(
        "mousemove", 
        closure.as_ref().unchecked_ref()
      )
      .expect("msg");

    closure.forget();    
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
        rel: "stylesheet",
        href: "https://fonts.googleapis.com/css2?family=Fjalla+One&family=Sixtyfour+Convergence&display=swap"
      }
    }

    div {
      // onmousemove: move |evt| {
      //   info!("{:#?}", evt);
      // },
      class: "min-h-screen",
      div {
        class: "first-screen w-full h-screen flex flex-col gap-4 justify-center items-center p-8",
        id: "first-screen",
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
              link: "https://medium.com/@kongphopkingpeth".to_string(),
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
            "I’m pursuing studies in Financial Engineering but have a strong interest in Computer Science,"
            br {  }
            "particularly in Blockchain technology and Full-Stack development."
            br { }
            "I’m also passionate about track distance running."
          }
        }
      }
      div {
        class: "second-screen w-full min-h-screen flex flex-col gap-4 justify-center items-center p-8",
        div {
          id: "first-screen",
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
          div {  
            class: "w-2/3 max-sm:w-full flex flex-col gap-6",
            id: "first-graph",
            p {  
              class: "text-center text-2xl",
              "Running"
            }
            TotalCard {  
              window_size: "max-sm",
              element: rsx!{
                TotaDatalCard {  
                  window_size: "max-sm",
                  data: total_distance.running_activity.to_string(),
                  unit: "activities"
                }
                TotaDatalCard {  
                  window_size: "max-sm",
                  data: format!("{:.3}", total_distance.distance),
                  unit: "km."
                }
                TotaDatalCard {  
                  window_size: "max-sm",
                  data: total_distance.running_day.to_string(),
                  unit: "days"
                }
              }
            }
            TotalCard {  
              window_size: "sm",
              element: rsx!{
                TotaDatalCard {  
                  window_size: "sm",
                  data: total_distance.running_activity.to_string(),
                  unit: "activities"
                }
                TotaDatalCard {  
                  window_size: "sm",
                  data: format!("{:.3}", total_distance.distance),
                  unit: "km."
                }
                TotaDatalCard {  
                  window_size: "sm",
                  data: total_distance.running_day.to_string(),
                  unit: "days"
                }
              }
            }
            div {  
              class: "flex flex-col gap-4 w-full h-full",
              div {  
                class: "flex flex-row justify-between",
                p { 
                  "2025 Febuary daily running distance"
                }
                p {  
                  "Selector"
                }
              }
              div {  
                class: "w-full h-60 flex flex-row-reverse gap-2 max-sm:gap-1 rotate-180 bg-white p-2 rounded-md opacity-90",
                for bar in month_daily_percentage {
                  div {
                    style: format!(
                      "width: 100%; height: {}%; background-color: #4b5563;",
                      bar.percentage
                    ),
                    class: " rounded-md z-10"
                  }
                }
              }
            }
          }
        }
      }
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
