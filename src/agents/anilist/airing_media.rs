use graphql_client::GraphQLQuery;

use crate::utils::duration::Duration;

type Json = std::collections::HashMap<String, bool>;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schema.json",
  query_path = "graphql/queries/AiringMedia.gql",
  response_derives = "Debug,Clone"
)]
pub struct AiringMedia;

pub use self::airing_media::*;

impl self::airing_media::AiringMediaPageMediaList {
  pub fn id(&self) -> i64 {
    self.media.as_ref().unwrap().id
  }

  pub fn total_episodes(&self) -> Option<i64> {
    self.media.as_ref()?.episodes
  }

  pub fn progress(&self) -> i64 {
    self.progress.unwrap()
  }

  pub fn set_progress(&mut self, progress: i64) {
    self.progress = Some(progress);
  }

  pub fn next_episode(&self) -> Option<i64> {
    Some(self.media.as_ref()?.next_airing_episode.as_ref()?.episode)
  }

  pub fn next_episode_time(&self) -> Option<i64> {
    Some(self.media.as_ref()?.next_airing_episode.as_ref()?.airing_at)
  }

  pub fn airing_in(&self) -> Option<String> {
    let current_time = (js_sys::Date::now() / 1000.0) as u64;
    let airing_at = self.media.as_ref()?.next_airing_episode.as_ref()?.airing_at as u64;

    if airing_at < current_time {
      return Some("-".to_owned());
    }

    let airing_in = airing_at - current_time;
    let airing_in = airing_in / (60) * (60);

    let airing_in: Duration = std::time::Duration::from_secs(airing_in).into();

    Some(format!("{}", airing_in))
  }

  pub fn cover(&self) -> Option<String> {
    Some(self.media.as_ref()?.cover_image.as_ref()?.extra_large.as_ref()?.clone())
  }

  pub fn streaming_site(&self) -> Option<String> {
    let links = self.media.as_ref()?.external_links.as_ref()?;

    for link in links.iter() {
      let link = link.as_ref()?;

      if link.site == "Funimation" {
        return Some(link.url.clone());
      }

      if link.site == "Crunchyroll" {
        return Some(link.url.clone());
      }
    }

    None
  }
}
