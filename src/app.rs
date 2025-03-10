#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
  rsx! {
    div {
      class: "",
      "hello world"
    }
  }
}