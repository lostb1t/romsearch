#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct File {
    id: u64,
    name: String,
    location: String,
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    let mut results: Signal<Vec<meilisearch_sdk::search::SearchResult<File>>> = use_signal(|| vec![]);
    let mut search = use_signal(|| "".to_string());

    let SEARCH_API_URL = env::var("SEARCH_API_URL").unwrap();
    let SEARCH_API_KEY = env::var("SEARCH_API_KEY").unwrap();
    
    rsx! {
        h1 { "Rom search" }
        // form { onsubmit,
        input {
            r#type: "text", id: "search", name: "search", oninput: move |evt| async move {
                search.set(evt.value());
                let client = Client::new(
                        SEARCH_API_URL,
                        Some(SEARCH_API_KEY)
                ).unwrap();
                //info!("called");
                let search_result = client.index("files").search().with_limit(50).with_query(search().as_str()).execute::<File>().await.unwrap();
                results.set(search_result.hits);
                //text.set(format!("{:?}", search_result.hits));
            }
            
           // move |evt| search.set(evt.value()),
        }
        button {
            onclick: move |_| async move {
                let client = Client::new(
                        SEARCH_API_URL,
                        Some(SEARCH_API_KEY)
                ).unwrap();
                let search_result = client.index("files").search().with_limit(50).with_query(search().as_str()).execute::<File>().await.unwrap();
                results.set(search_result.hits);
                //text.set(format!("{:?}", search_result.hits));
            },
            "Search"
        }
        ul { class: "todo-list",
            for result in results.iter() {
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

fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    // #[cfg(feature = "server")]
    // tracing_subscriber::fmt::init();
    info!("Starting romsearch");
    dioxus::launch(app);

    //println!("{:?}", client.index("files").search().with_query("caorl").execute::<File>().await.unwrap().hits);
}
