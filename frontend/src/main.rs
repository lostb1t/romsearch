#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct File {
    id: u64,
    name: String,
    location: String,
}

async fn search(input: &str) -> Vec<meilisearch_sdk::search::SearchResult<File>> {
    let SEARCH_API_URL: &'static str = env!("SEARCH_API_URL");
    let SEARCH_API_KEY: &'static str = env!("SEARCH_API_KEY");
    //let SEARCH_API_URL = env::var("SEARCH_API_URL").unwrap();
    //let SEARCH_API_KEY = env::var("SEARCH_API_KEY").unwrap();
    let client = Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap();
    
    client
        .index("files")
        .search()
        .with_limit(50)
        .with_query(input)
        .execute::<File>()
        .await
        .unwrap()
        .hits
}

fn app() -> Element {
    //let mut results: Signal<Vec<meilisearch_sdk::search::SearchResult<File>>> =
    // use_signal(|| vec![]);
    let mut input = use_signal(|| "".to_string());
    
    let results = use_resource(move || async move { 
      if &input() == "" {
        return None;
      }
      Some(search(&input()).await)
    });
    // info!("YOHO");
    //let client = Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap();

    rsx! {
        h1 { "Rom search" }
        // form { onsubmit,
        input {
            r#type: "text",
            id: "search",
            name: "search",
            oninput: move |evt| {
                // let s = client.clone();

                    // let SEARCH_API_URL = env::var("SEARCH_API_URL").unwrap();
                    // let SEARCH_API_KEY = env::var("SEARCH_API_KEY").unwrap();
                    info!("CALLED");
                    input.set(evt.value());
                    //let client = Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap();
                    // let search_result = client.index("files").search().with_limit(50).with_query(evt.value().as_str()).execute::<File>().await.unwrap();
                    // results.set(search_result.hits);

            }

           // move |evt| search.set(evt.value()),
        }

        if let Some(Some(r)) = results.read().as_ref() {
        ul { class: "todo-list",
            for result in r.iter() {
                li {
                    a {
                      href: {result.result.location.clone()},
                      target: "_blank",
                      {result.result.name.clone()}
                    }
                }
            }
        }
    }
    }
}

fn main() {
    // #[cfg(feature = "web")]
    // tracing_wasm::set_as_global_default();
    dioxus_logger::init(Level::INFO).expect("logger failed to init");

    // #[cfg(feature = "server")]
    // tracing_subscriber::fmt::init();
    info!("Starting romsearch");
    dioxus::launch(app);

    //println!("{:?}", client.index("files").search().with_query("caorl").execute::<File>().await.unwrap().hits);
}
