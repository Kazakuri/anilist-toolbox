use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod airing;
pub mod auth;
pub mod home;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "/auth/logout"]
  Logout,
  #[to = "/auth"]
  Auth,
  #[to = "/airing"]
  Airing,
  #[to = "/page-not-found"]
  PageNotFound(Permissive<String>),
  #[to = "/"]
  Home,
}
