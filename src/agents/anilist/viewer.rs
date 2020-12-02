use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schema.json",
  query_path = "graphql/queries/Viewer.gql",
  response_derives = "Debug,Clone"
)]
pub struct Viewer;

pub use self::viewer::*;
