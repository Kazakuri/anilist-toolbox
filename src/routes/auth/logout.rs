use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

pub struct Logout;

impl Component for Logout {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

    let window = web_sys::window().unwrap();

    match storage.restore("access_token") {
      Ok(_) => {
        storage.remove("access_token");
        if let Err(e) = window.location().reload() {
          log::error!("{:?}", e);
        }
      },
      Err(_) => {
        if let Err(e) = window.location().set_href("/") {
          log::error!("{:?}", e);
        }
      }
    }

    Self {}
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {}
  }
}
