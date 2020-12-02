use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schema.json",
  query_path = "graphql/mutations/UpdateMedia.gql",
  response_derives = "Debug,Clone"
)]
pub struct UpdateMedia;

pub use self::update_media::*;
