use std::collections::BTreeMap;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::anilist;
use crate::agents::anilist::AniList;
use crate::components::media::Media;

pub struct Airing {
  media_list: Option<BTreeMap<String, Vec<anilist::AiringMediaList>>>,
  _anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
}

pub enum Msg {
  AniListMsg(ReadOnly<AniList>),
}

impl Component for Airing {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::AniListMsg);
    let mut anilist = AniList::bridge(callback);

    anilist.send(anilist::Request::FetchAiringMedia);

    Self {
      media_list: None,
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

        match &state.airing_media {
          Some(v) => {
            let mut map: BTreeMap<String, Vec<anilist::AiringMediaList>> = BTreeMap::new();
            map.insert("Watching".to_owned(), vec![]);

            for m in v.iter() {
              if !m.hidden_from_status_lists.unwrap_or(false) {
                if let Some(x) = map.get_mut("Watching") {
                  x.push(m.clone());
                }
              }
              for (list, on_list) in m.custom_lists.as_ref().unwrap().iter() {
                if !map.contains_key(list) {
                  map.insert(list.to_owned(), vec![]);
                }

                if let Some(x) = map.get_mut(list) {
                  if *on_list {
                    x.push(m.clone());
                  }
                }
              }
            }

            for (_, list) in map.iter_mut() {
              list.sort_by(|l, r| {
                l.next_episode_time()
                  .unwrap()
                  .partial_cmp(&r.next_episode_time().unwrap())
                  .unwrap()
              });
            }

            self.media_list = Some(map);
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
          match &self.media_list {
            None => html! {},
            Some(list) => html! {
              <>
                {
                  for list.iter().map(|(list, media)| {
                    if media.len() > 0 {
                      html! {
                        <>
                          <h1>
                            { list }
                          </h1>
                          <div class="grid">
                          {
                            for media.iter().map(|i| html! {
                              <Media media={i} key={i.id()} />
                            })
                          }
                          </div>
                        </>
                      }
                    } else {
                      html! {}
                    }
                  })
                }
              </>
            },
          }
        }
      </div>
    }
  }
}
