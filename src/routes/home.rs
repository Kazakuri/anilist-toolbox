use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::anilist;
use crate::agents::anilist::AniList;

pub struct Home {
  viewer: Option<anilist::Viewer>,
  _anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
}

pub enum Msg {
  AniListMsg(ReadOnly<AniList>),
}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::AniListMsg);
    let mut anilist = AniList::bridge(callback);

    anilist.send(anilist::Request::FetchViewer);

    Self {
      viewer: None,
      _anilist: anilist,
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Self::Message::AniListMsg(state) => {
        let state = state.borrow();

        match &state.viewer {
          Some(v) => {
            self.viewer = Some(v.clone());
            true
          }
          _ => false,
        }
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <div class="app">
        {
          match &self.viewer {
            Some(viewer) => html! {
              <h1>
                { format!("Welcome Back, {}!", &viewer.name) }
              </h1>
            },
            None => html!{},
          }
        }
      </div>
    }
  }
}
