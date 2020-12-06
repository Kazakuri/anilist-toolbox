use std::collections::BTreeMap;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use yew::services::interval::{IntervalService, IntervalTask};
use std::time::Duration;

use crate::agents::anilist;
use crate::agents::anilist::AniList;
use crate::components::media::Media;

pub struct Airing {
  media_list: Option<BTreeMap<String, Vec<anilist::AiringMediaList>>>,
  anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
  _interval: IntervalTask,
}

pub enum Msg {
  AniListMsg(ReadOnly<AniList>),
  UpdateTime,
}

impl Component for Airing {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::AniListMsg);
    let mut anilist = AniList::bridge(callback);

    anilist.send(anilist::Request::FetchAiringMedia);

    // It would be more accurate to do this every second, but it wastes more CPU cycles and I don't care
    let interval = IntervalService::spawn(
      Duration::from_secs(60), 
      link.callback(|_| Msg::UpdateTime)
    );

    Self {
      media_list: None,
      anilist: anilist,
      _interval: interval,
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Self::Message::UpdateTime => { 
        let dash = Some("-".to_owned());
        let has_aired = |m: &BTreeMap<String, Vec<anilist::AiringMediaList>>| {
          m.values().any(|m| m.iter().any(|m| m.airing_in() == dash))
        };

        if self.media_list.iter().any(has_aired) {
          self.anilist.send(anilist::Request::FetchAiringMedia);
        }
        true
      },
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
