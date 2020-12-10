use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::anilist;
use crate::agents::anilist::AniList;
use crate::components::media::Media;

pub struct Home {
  viewer: Option<anilist::Viewer>,
  _anilist: Box<dyn Bridge<StoreWrapper<AniList>>>,
  media: anilist::AiringMediaList,
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

    let media = anilist::AiringMediaList {
      progress: Some(5),
      custom_lists: None,
      hidden_from_status_lists: Some(false),
      media: Some(anilist::airing_media::AiringMediaPageMediaListMedia {
        id: 21379,
        title: Some(anilist::airing_media::AiringMediaPageMediaListMediaTitle {
          english: Some("Fate/kaleid liner Prisma☆Illya 3rei!!".to_string()),
          romaji: Some("Fate/kaleid liner Prisma☆Illya 3rei!!".to_string()),
          native: Some("Fate/kaleid liner プリズマ☆イリヤ ドライ!!".to_string()),
        }),
        episodes: Some(12),
        cover_image: Some(anilist::airing_media::AiringMediaPageMediaListMediaCoverImage {
          extra_large: Some("/21379.4051ca79.png".to_string()),
        }),
        external_links: Some(vec! [
          Some(anilist::airing_media::AiringMediaPageMediaListMediaExternalLinks {
            site: "Crunchyroll".to_string(),
            url: "https://www.crunchyroll.com/fatekaleid-liner-prisma-illya".to_string(),
          })
        ]),
        next_airing_episode: Some(anilist::airing_media::AiringMediaPageMediaListMediaNextAiringEpisode {
          airing_at: (js_sys::Date::now() / 1000.0) as i64 + 198700,
          episode: 9,
        }),
      })
    };

    Self {
      viewer: None,
      _anilist: anilist,
      media,
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
              <div class="introduction">
                <h1>
                  { format!("Welcome Back, {}!", &viewer.name) }
                </h1>
                <h2>
                  { "Airing" }
                </h2>
                <span>
                  { "The airing page extends the basic AniList airing panel with the following new features:"}
                </span>
                <ul>
                  <li> { "Group series by custom lists" } </li>
                  <li> { "Always show all airing series, instead of just the first page of watching series" } </li>
                  <li> { "Show at a glance how far behind on a series you are" } </li>
                  <li> { "Link to a streaming site to quickly catch up" } </li>
                </ul>
                <div class="grid example">
                  <Media media={&self.media} disabled={true} />
                </div>
                <div class="arrow_overlay">
                  <div style="position: absolute; left: calc(50% - 530px); top: 106px">
                    { "Current Progress + Episodes Behind" }
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 152.015 274.2422" width="75px" style="position: absolute; left: calc(50% - 230px); top: 76px; transform: rotate(60deg);">
                    <path d="M167.1359,92.4005a672.35,672.35,0,0,1-22.3753-73.1554c-1.7724-7.2672-12.4234-10.6788-17.5388-4.6305q-21.9289,25.9281-43.8674,51.848c-8.68,10.2585,4.8761,24.3284,14.9122,14.9124l17.7828-16.6841A357.6331,357.6331,0,0,1,54.187,226.43c-10.8308,15.5677-23.0813,29.9418-35.9779,43.8262-7.9878,8.6,4.2941,20.9058,12.8826,12.8826,51.5229-48.1319,86.3119-115.4956,100.7331-184.1214q3.14-14.9422,5.0149-30.1137,4.6112,14.6667,9.8656,29.1294c1.9578,5.373,7.196,9.0028,13.0316,7.3991C164.9025,104.0125,169.1241,97.7959,167.1359,92.4005Z" transform="translate(-15.6225 -11.4988)"/>
                  </svg>

                  <div style="position: absolute; left: calc(50% + 230px); top: 106px">
                    { "Increment Progress" }
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 152.015 274.2422" width="75px" style="position: absolute; left: calc(50% + 140px); top: 76px; transform: scale(-1, 1) rotate(60deg);">
                    <path d="M167.1359,92.4005a672.35,672.35,0,0,1-22.3753-73.1554c-1.7724-7.2672-12.4234-10.6788-17.5388-4.6305q-21.9289,25.9281-43.8674,51.848c-8.68,10.2585,4.8761,24.3284,14.9122,14.9124l17.7828-16.6841A357.6331,357.6331,0,0,1,54.187,226.43c-10.8308,15.5677-23.0813,29.9418-35.9779,43.8262-7.9878,8.6,4.2941,20.9058,12.8826,12.8826,51.5229-48.1319,86.3119-115.4956,100.7331-184.1214q3.14-14.9422,5.0149-30.1137,4.6112,14.6667,9.8656,29.1294c1.9578,5.373,7.196,9.0028,13.0316,7.3991C164.9025,104.0125,169.1241,97.7959,167.1359,92.4005Z" transform="translate(-15.6225 -11.4988)"/>
                  </svg>

                  <div style="position: absolute; left: calc(50% + 170px); top: 26px">
                    { "Stream Episode" }
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 152.015 274.2422" width="75px" style="position: absolute; left: calc(50% + 50px);  transform: rotate(220deg);">
                    <path d="M167.1359,92.4005a672.35,672.35,0,0,1-22.3753-73.1554c-1.7724-7.2672-12.4234-10.6788-17.5388-4.6305q-21.9289,25.9281-43.8674,51.848c-8.68,10.2585,4.8761,24.3284,14.9122,14.9124l17.7828-16.6841A357.6331,357.6331,0,0,1,54.187,226.43c-10.8308,15.5677-23.0813,29.9418-35.9779,43.8262-7.9878,8.6,4.2941,20.9058,12.8826,12.8826,51.5229-48.1319,86.3119-115.4956,100.7331-184.1214q3.14-14.9422,5.0149-30.1137,4.6112,14.6667,9.8656,29.1294c1.9578,5.373,7.196,9.0028,13.0316,7.3991C164.9025,104.0125,169.1241,97.7959,167.1359,92.4005Z" transform="translate(-15.6225 -11.4988)"/>
                  </svg>
                  
                  <div style="position: absolute; left: calc(50% - 280px); top: 346px">
                    { "Next Episode" }
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 152.015 274.2422" width="75px" style="position: absolute; left: calc(50% - 180px); top: 316px; transform: rotate(60deg);">
                    <path d="M167.1359,92.4005a672.35,672.35,0,0,1-22.3753-73.1554c-1.7724-7.2672-12.4234-10.6788-17.5388-4.6305q-21.9289,25.9281-43.8674,51.848c-8.68,10.2585,4.8761,24.3284,14.9122,14.9124l17.7828-16.6841A357.6331,357.6331,0,0,1,54.187,226.43c-10.8308,15.5677-23.0813,29.9418-35.9779,43.8262-7.9878,8.6,4.2941,20.9058,12.8826,12.8826,51.5229-48.1319,86.3119-115.4956,100.7331-184.1214q3.14-14.9422,5.0149-30.1137,4.6112,14.6667,9.8656,29.1294c1.9578,5.373,7.196,9.0028,13.0316,7.3991C164.9025,104.0125,169.1241,97.7959,167.1359,92.4005Z" transform="translate(-15.6225 -11.4988)"/>
                  </svg>

                  <div style="position: absolute; left: calc(50% + 140px); top: 473px;">
                    { "Behind Marker" }
                  </div>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 152.015 274.2422" width="75px" style="position: absolute; left: calc(50% + 20px); top: 406px; transform: scale(-1, 1) rotate(50deg);">
                    <path d="M167.1359,92.4005a672.35,672.35,0,0,1-22.3753-73.1554c-1.7724-7.2672-12.4234-10.6788-17.5388-4.6305q-21.9289,25.9281-43.8674,51.848c-8.68,10.2585,4.8761,24.3284,14.9122,14.9124l17.7828-16.6841A357.6331,357.6331,0,0,1,54.187,226.43c-10.8308,15.5677-23.0813,29.9418-35.9779,43.8262-7.9878,8.6,4.2941,20.9058,12.8826,12.8826,51.5229-48.1319,86.3119-115.4956,100.7331-184.1214q3.14-14.9422,5.0149-30.1137,4.6112,14.6667,9.8656,29.1294c1.9578,5.373,7.196,9.0028,13.0316,7.3991C164.9025,104.0125,169.1241,97.7959,167.1359,92.4005Z" transform="translate(-15.6225 -11.4988)"/>
                  </svg>
                </div>
              </div>
            },
            None => html!{},
          }
        }
      </div>
    }
  }
}
