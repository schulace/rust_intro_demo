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


#[derive(Deserialize, Debug)]
struct ActionsQueryResponse {
  data: ActionsQueryAction
}

#[derive(Deserialize, Debug)]
struct ActionsQueryEdges {
  edges: Vec<ActionsQueryNode>
}

#[derive(Deserialize, Debug)]
struct ActionsQueryAction {
  actions: ActionsQueryEdges
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
  deadline: Option<DateTime<Utc>>,
}


static GRAPHQL_ENDPOINT: &'static str  = "https://prod-gql.hive.com/graphql";
static ACTIONS_QUERY: &'static str = r#"{"query": "query { actions { edges { node { _id, title, status, deadline, } } } }"}"#;

struct HiveGraphqlClient {
  client: reqwest::Client
}


impl HiveGraphqlClient {
  fn new(jwt: &str) -> HiveGraphqlClient {
    let default_headers: HeaderMap = vec![
      (HeaderName::from_static("authorization"), HeaderValue::from_str(&format!("Bearer {}", jwt)).expect("malformed JWT")),
      (HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"))
    ].into_iter().collect();
    
    HiveGraphqlClient {
      client: ClientBuilder::new().default_headers(default_headers).build().expect("couldn't initialize TLS"),
    }
  }
  async fn my_overdue_actions(&self) -> Result<impl Iterator<Item = ActionNode>> {
    let client = &self.client;
    let res_data: ActionsQueryResponse = client.post(GRAPHQL_ENDPOINT).body(ACTIONS_QUERY).send().await?.json().await?;
    Ok(res_data.data.actions.edges.into_iter()
      .map(|q_node| q_node.node)
      .filter(|node| match node.deadline {
        Some(deadline) if deadline > Utc::now() => true,
        _ => false
      }))
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = HiveGraphqlClient::new("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJfaWQiOiJiQnRCdFA2eEV3WnJCZ3JaTSIsImVtYWlsIjoiYWxleC5zY2h1bGVyQGhpdmUuY29tIiwiaWF0IjoxNTg5ODI4OTk3fQ.F2OzcFFhwa-JVuwub-afjivQ_1pZcRI5mLpphvdz4cM");
  for action in client.my_overdue_actions().await? {
    println!("{:?}", action);
  }
  Ok(())

}

// #[tokio::main]
// async fn main() -> reqwest::Result<()> {
//   let client = reqwest::Client::new();

//   //does nothing
//   tokio::spawn(print_site("https://www.yahoo.com"));

//   //prints the contents of yahoo.com to the screen
//   // print_site("https://www.google.com").await;
//   Ok(())
// }
// async fn print_site(site: &str) -> reqwest::Result<()>{
//   let s = reqwest::get(site).await?.text().await?;
//   println!("{}", s);
//   Ok(())
// }

