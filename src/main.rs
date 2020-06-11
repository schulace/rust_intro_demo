#![allow(warnings)]
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio::{self, io::AsyncReadExt};
use std::time::SystemTime;
use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  ClientBuilder,
};
use anyhow::{Context, Result};

static GRAPHQL_ENDPOINT: &'static str  = "https://prod-gql.hive.com/graphql";
static ACTIONS_QUERY: &'static str = r#"{"query": "
  query {
    actions {
      edges {
        node {
          _id,
          title,
          status,
          deadline
        }
      }
    }
  }
"}"#;

/// Top level response. The following structs mirror the structure of
/// the query.
#[derive(Deserialize, Debug)]
struct ActionsQueryResponse {
  data: ActionsQueryAction
}
#[derive(Deserialize, Debug)]
struct ActionsQueryAction {
  actions: ActionsQueryEdges
}

#[derive(Deserialize, Debug)]
struct ActionsQueryEdges {
  edges: Vec<ActionsQueryNode>
}


#[derive(Deserialize, Debug)]
struct ActionsQueryNode {
  node: ActionNode
}

#[derive(Deserialize, Debug)]
struct ActionNode {
  _id: String,
  title: String,
  status: String,
  /// Optional fields in GQL are represented with `Option`
  deadline: Option<DateTime<Utc>>,
}

/// A wrapper around an HTTP client that will exclusively connect to prod-gql
struct HiveGraphqlClient {
  client: reqwest::Client
}


impl HiveGraphqlClient {
  fn new(jwt: &str) -> HiveGraphqlClient {
    /**
     * To connect to GQL, we need to have headers
     *  authorization="Bearer ${jwt}"
     *  content-type="application/json"
     *  
     * HeaderMap implements FromIterator<Item=(HeaderName, HeaderValue)>
     */
    let default_headers: HeaderMap = vec![
      unimplemented!(),
      unimplemented!(),
    ].into_iter().collect();
    
    HiveGraphqlClient {
      client: ClientBuilder::new().default_headers(default_headers).build().expect("couldn't initialize TLS"),
    }
  }
  async fn my_overdue_actions(&self) -> Result<impl Iterator<Item = ActionNode>> {
    let client = &self.client;
    //use the client to POST to the endpoint, set the body to the query, send the query, and receive it as JSON
    // which will parse directly into an ActionQueryResponse
    let res_data: ActionsQueryResponse = unimplemented!();

    // return an Ok with an iterator over the action nodes
    Ok(res_data.data.actions.edges.into_iter()
      // go from { node: {_id, deadline, ...}} to {_id, deadline, ...}
      .map(|q_node| q_node.node)
      // return only results where the deadline exists and is after right now
      .filter(|node| unimplemented!()))
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = HiveGraphqlClient::new(unimplemented!("your JWT here"));
  for action in client.my_overdue_actions().await? {
    println!("{:?}", action);
  }
  Ok(())

}


