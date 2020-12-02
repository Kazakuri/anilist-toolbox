use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::anilist;
use crate::agents::anilist::AniList;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav {
  viewer: Option<anilist::Viewer>,
  _anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
  client_id: u32,
}

pub enum Msg {
  AniListMsg(ReadOnly<AniList>),
}

impl Component for Nav {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::AniListMsg);
    let mut anilist = AniList::bridge(callback);

    anilist.send(anilist::Request::FetchViewer);

    let client_id = if cfg!(debug_assertions) {
      4152
    } else {
      4440
    };

    Self {
      viewer: None,
      _anilist: anilist,
      client_id,
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
      <nav class="flex items-center justify-between flex-wrap bg-white dark:bg-gray-900 py-2 lg:px-12 shadow border-solid border-t-2 border-blue-700">
        <div class="flex justify-between lg:w-auto w-full lg:border-b-0 pl-6 pr-2 border-solid border-b-2 border-gray-300 pb-5 lg:pb-0">
          <div class="flex items-center flex-shrink-0 text-gray-800 dark:text-gray-200 mr-16">
            <span class="font-semibold text-xl tracking-tight">{ "AniList Toolbox" }</span>
          </div>
          <div class="block lg:hidden ">
            <button id="nav"
              class="flex items-center px-3 py-2 border-2 rounded text-blue-700 border-blue-700 hover:text-blue-700 hover:border-blue-700">
              <svg class="fill-current h-3 w-3" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                <title>{ "Menu" }</title>
                <path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" />
              </svg>
            </button>
          </div>
        </div>

        <div class="menu w-full lg:block flex-grow lg:flex lg:items-center lg:w-auto lg:px-3 px-8">
          <div class="text-base lg:flex-grow">
            <RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link">
              { "Home" }
            </RouterAnchor<AppRoute>>
            {
              match &self.viewer {
                Some(viewer) => html! {
                  <RouterAnchor<AppRoute> route=AppRoute::Airing classes="app-link">
                    { "Airing" }
                  </RouterAnchor<AppRoute>>
                },
                None => html! {}
              }
            }
          </div>
          <div class="flex">
            {
              match &self.viewer {
                Some(viewer) => html! {
                  <>
                  <RouterAnchor<AppRoute> route=AppRoute::Logout classes="login-link">
                    { "Logout" }
                  </RouterAnchor<AppRoute>>
                    <a href={ format!("https://anilist.co/user/{}", &viewer.name) }>
                      {
                        match &viewer.avatar {
                          Some(avatar) => match &avatar.large {
                            Some(img) => html! {
                              <img class="h-12 inline-block" src={ img } />
                            },
                            _ => html! {}
                          },
                          _ => html! {}
                        }
                      }
                    </a>
                  </>
                },
                None => html! {
                  <a class="app-link" href={ format!("https://anilist.co/api/v2/oauth/authorize?client_id={}&response_type=token", self.client_id) }>
                    { "Login" }
                  </a>
                },
              }
            }
          </div>
        </div>
      </nav>
    }
  }
}
