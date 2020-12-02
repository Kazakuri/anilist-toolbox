use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::prelude::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Fragment {
  pub access_token: Option<String>,
  pub token_type: Option<String>,
  pub expires_in: Option<u32>,
}

pub mod logout;

pub struct Auth;

impl Component for Auth {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let route: RouteService<()> = RouteService::new();
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

    let fragment = route.get_fragment();

    if !fragment.is_empty() {
      if let Ok(fragment) = serde_qs::from_str::<Fragment>(&fragment[1..]) {
        if let Some(t) = fragment.access_token {
          storage.store("access_token", Ok(t));
        }
      }
    }

    let window = web_sys::window().unwrap();
    
    if let Err(e) = window.location().set_href("/") {
      log::error!("{:?}", e);
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
