use web_sys::{window, Window};

pub struct WindowData {
  pub window: Window,
}

impl WindowData {
  pub fn new() -> Self {
    let window = window().expect("There should be window");
    Self { window }
  }

  pub fn screen_size(&self) -> (i32, i32) {
    let width = self.window
    .inner_width()
    .expect("The window should have Some width")
    .as_f64()
    .expect("The width should be a number") as i32;

  let height = self.window
    .inner_height()
    .expect("The window should have Some width")
    .as_f64()
    .expect("The width should be a number") as i32;
    
    (width, height)
  }

  pub fn element_id_size(&self, id: &str) -> (i32, i32) {
    let element = self.window
      .document()
      .expect("There should be document")
      .get_element_by_id(id)
      .expect("There should be id element");
    
    let width = element.client_width();
    let height = element.client_height();

    (width, height)
  }
}