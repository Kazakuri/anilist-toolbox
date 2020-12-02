use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
use yew::agent::AgentLink;
use yew::services::storage::{Area, StorageService};
use yewtil::future::LinkFuture;
use yewtil::store::{Store, StoreWrapper};

use jsonwebtoken::{
  // Note: This is a really stupid name, it just means we don't validate the signature.
  // This is pretty much irrelevant to us because the AniList server is responsible for that.
  dangerous_insecure_decode as validate,
  // Validation,
  TokenData,
};

pub mod airing_media;
pub mod update_media;
pub mod viewer;

pub type Viewer = viewer::ViewerViewer;
pub type AiringMediaList = airing_media::AiringMediaPageMediaList;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  aud: String,
  exp: usize,
  iat: usize,
  nbf: usize,
  sub: String,
}

pub struct AniList {
  pub viewer: Option<viewer::ViewerViewer>,
  pub airing_media: Option<Vec<airing_media::AiringMediaPageMediaList>>,

  has_requested_viewer: bool,
  has_requested_airing_media: bool,

  access_token: Option<String>,
  claims: Option<TokenData<Claims>>,
}

#[derive(Debug)]
pub enum Request {
  FetchViewer,
  FetchAiringMedia,
  UpdateMediaProgress(i64, i64),
}

#[derive(Debug)]
pub enum Action {
  NoAction,
  ApiError(reqwest::Error),
  RequestingViewer,
  SetViewer(Option<viewer::ViewerViewer>),
  RequestingAiringMedia,
  SetAiringMedia(Option<Vec<Option<airing_media::AiringMediaPageMediaList>>>),
  UpdateMediaProgress(i64, i64),
}

impl Store for AniList {
  type Action = Action;
  type Input = Request;

  fn new() -> Self {
    let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

    // TODO: We can't validate claims yet.
    // We can't get the time on `wasm32-unknown-unknown`: https://github.com/rust-lang/rust/issues/48564
    // We can't switch to `wasm32-wasi` since `wasm-pack` doesn't support it yet: https://github.com/rustwasm/wasm-pack/issues/654
    // Note: When this changes, we also need to change `dangerous_insecure_decode` to `dangerous_insecure_decode_with_validation`
    //let validation = Validation::new(jsonwebtoken::Algorithm::RS256);

    let (access_token, claims) = match storage.restore("access_token") {
      Ok(t) => match validate::<Claims>(&t /* &validation */) {
        Ok(claims) => (Some(t), Some(claims)),
        Err(e) => {
          log::error!("{}", e);
          (None, None)
        }
      },
      Err(e) => {
        // This probably just means the access_token wasn't found.
        // We don't care.
        (None, None)
      }
    };

    AniList {
      viewer: None,
      airing_media: None,
      has_requested_viewer: false,
      has_requested_airing_media: false,
      access_token,
      claims,
    }
  }

  fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
    let access_token = match &self.access_token {
      Some(t) => t.clone(),
      None => "".to_string(),
    };

    if access_token.is_empty() {
      return;
    }

    match msg {
      Self::Input::FetchViewer => {
        if !self.has_requested_viewer {
          link.send_message(Self::Action::RequestingViewer);
          link.send_future(async move {
            let client = reqwest::Client::new();

            let request_body = viewer::Viewer::build_query(viewer::Variables {});

            let req = client
              .post("https://graphql.anilist.co")
              .header("Authorization", format!("Bearer {}", access_token))
              .json(&request_body)
              .send()
              .await;

            match req {
              Ok(res) => match res.json::<graphql_client::Response<viewer::ResponseData>>().await {
                Ok(response) => Self::Action::SetViewer(response.data.unwrap().viewer),
                Err(e) => Self::Action::ApiError(e),
              },
              Err(e) => Self::Action::ApiError(e),
            }
          });
        } else {
          link.send_message(Self::Action::NoAction);
        }
      }
      Self::Input::FetchAiringMedia => {
        if !self.has_requested_airing_media {
          link.send_message(Self::Action::RequestingAiringMedia);

          let user = self.claims.as_ref().unwrap().claims.sub.parse::<i64>().unwrap();

          link.send_future(async move {
            let client = reqwest::Client::new();

            let request_body = airing_media::AiringMedia::build_query(airing_media::Variables {
              page: Some(1),
              user: Some(user),
            });

            let req = client
              .post("https://graphql.anilist.co")
              .header("Authorization", format!("Bearer {}", access_token))
              .json(&request_body)
              .send()
              .await;

            match req {
              Ok(res) => match res.json::<graphql_client::Response<airing_media::ResponseData>>().await {
                Ok(response) => Self::Action::SetAiringMedia(response.data.unwrap().page.unwrap().media_list),
                Err(e) => Self::Action::ApiError(e),
              },
              Err(e) => Self::Action::ApiError(e),
            }
          });
        } else {
          link.send_message(Self::Action::NoAction);
        }
      }
      Self::Input::UpdateMediaProgress(id, progress) => {
        link.send_future(async move {
          let client = reqwest::Client::new();

          let request_body = update_media::UpdateMedia::build_query(update_media::Variables {
            media_id: id,
            progress: Some(progress),
          });

          let req = client
            .post("https://graphql.anilist.co")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&request_body)
            .send()
            .await;

          match req {
            Ok(res) => match res.json::<graphql_client::Response<update_media::ResponseData>>().await {
              Ok(_) => Self::Action::UpdateMediaProgress(id, progress),
              Err(e) => Self::Action::ApiError(e),
            },
            Err(e) => Self::Action::ApiError(e),
          }
        });
      }
    }
  }

  fn reduce(&mut self, msg: Self::Action) {
    match msg {
      Self::Action::NoAction => (),
      Self::Action::ApiError(e) => log::error!("{}", e),
      Self::Action::RequestingViewer => self.has_requested_viewer = true,
      Self::Action::SetViewer(v) => self.viewer = v,
      Self::Action::RequestingAiringMedia => self.has_requested_airing_media = true,
      Self::Action::SetAiringMedia(m) => self.airing_media = m.map(|m| m.into_iter().map(|m| m.unwrap()).collect()),
      Self::Action::UpdateMediaProgress(id, progress) => {
        if let Some(media_list) = &mut self.airing_media {
          for media in media_list.iter_mut() {
            if media.id() == id {
              media.set_progress(progress);
              return;
            }
          }
        }
      }
    }
  }
}
