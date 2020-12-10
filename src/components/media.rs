use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::anilist;
use crate::agents::anilist::AniList;

/// Media component
pub struct Media {
  props: Props,
  link: ComponentLink<Self>,
  anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
}

pub enum Msg {
  AniListMsg(ReadOnly<AniList>),
  AddProgress,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub media: anilist::AiringMediaList,
  #[prop_or_default]
  pub disabled: bool,
}

impl Component for Media {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::AniListMsg);
    let anilist = AniList::bridge(callback);

    Self { props, link, anilist }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    if self.props.disabled {
      return true;
    }

    match msg {
      Self::Message::AniListMsg(_) => true,
      Self::Message::AddProgress => {
        self.anilist.send(anilist::Request::UpdateMediaProgress(
          self.props.media.id(),
          self.props.media.progress() + 1,
        ));

        false
      }
    }
  }

  fn view(&self) -> Html {
    html! {
      <div class="img" style={ format!("background-image: url({})", self.props.media.cover().unwrap()) }>
        {
          if self.props.media.next_episode().is_some() && self.props.media.progress.unwrap() < self.props.media.next_episode().unwrap() - 1 {
            html! {
              <div class="behind"></div>
            }
          } else {
            html! {}
          }
        }
        <a href={ format!("https://anilist.co/anime/{}", self.props.media.id()) } target="_blank" />
        <div class="overlay top">
          <div class="text">
            {
              self.props.media.progress.unwrap()
            }
            <span class="behind_count">
              {
                match self.props.media.next_episode() {
                  Some(next) => if next - self.props.media.progress.unwrap() > 1 {
                    html! {
                      <>
                        { format!("({})", next - 1 - self.props.media.progress.unwrap()) }
                        <br />
                      </>
                    }
                  } else {
                    html! {}
                  },
                  None => match self.props.media.total_episodes() {
                    Some(total) => if total - self.props.media.progress.unwrap() > 0 {
                      html! {
                        <>
                          { format!("({})", total - self.props.media.progress.unwrap()) }
                          <br />
                        </>
                      }
                    } else {
                      html! {}
                    },
                    None => html! {}
                  }
                }
              }
            </span>
          </div>
          <div>
            {
              match self.props.media.streaming_site() {
                Some(url) => html! {
                  <a href={url} target="_blank">
                    <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                      <circle cx="12" cy="12" r="3"></circle>
                    </svg>
                  </a>
                },
                None => html! {}
              }
            }
          </div>
          <div>
            <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round" onclick=self.link.callback(|_| Msg::AddProgress)>
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
          </div>
        </div>
        {
          match self.props.media.next_episode() {
            Some(next) => html! {
              <div class="overlay">
                {
                  match self.props.media.total_episodes() {
                    Some(total) => html! {
                      <>
                        { format!("Ep {} of {}", next, total) }
                        <br />
                        { format!("{}", self.props.media.airing_in().unwrap()) }
                      </>
                    },
                    None => html! {
                      <>
                        { format!("Ep {}", next) }
                        <br />
                        { format!("{}", self.props.media.airing_in().unwrap()) }
                      </>
                    }
                  }
                }
              </div>
            },
            None => html! {}
          }
        }
      </div>
    }
  }
}
