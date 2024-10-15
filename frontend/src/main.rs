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
    size: String,
    date: String,
}

async fn search(input: &str) -> Vec<meilisearch_sdk::search::SearchResult<File>> {
    let SEARCH_API_URL: &'static str = env!("SEARCH_API_URL");
    let SEARCH_API_KEY: &'static str = env!("SEARCH_API_KEY");
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
    let mut input = use_signal(|| "".to_string());

    let results = use_resource(move || async move {
        if &input() == "" {
            return None;
        }
        Some(search(&input()).await)
    });

    rsx! {
            // head::Link {
            //     rel: "stylesheet",
            //     href: "https://unpkg.com/terminal.css@0.7.4/dist/terminal.min.css"
            // }
            // head::Link {
            //     rel: "stylesheet",
            //     href: asset!("./assets/site.css")
            // }
            // head::Style {
            //  r#"
            //      table td, table th {{
            //          border: 0;
            //      }}
            //  "#
            // }
            div {
                class: "container",

            h1 { "Game rom search" }
            input {
                r#type: "text",
                id: "search",
                name: "search",
                placeholder: "Search a game...",
                oninput: move |evt| {
                        input.set(evt.value());
                }
            }

            div {
                // style: "margin-top: 1em",
                style: "overflow-x:auto;",


                    table {
                        //style: "overflow-x:auto;",
                        tfoot {
                            tr {
                                th {
                                    colspan: 2,
                                    // p {
                                        // style: "font-size: 0.6em",
                                        "Powered by "
                                        a { href: "https://myrient.erista.me", target: "_blank", "Myrient" }
                                    // }
                                }
                            }
                        }
                        tr {
                            th { "name" }
                            th { "size" }
                            //th { "date" }
                        }
                        if let Some(Some(r)) = results.read().as_ref() {
                        if r.is_empty() {
                            tr {
                              td {  colspan: 2, "No results"}
                            }
                        } else {
                            for result in r.iter() {
                                tr {
                                    td {
                                    a {
                                    href: {result.result.location.clone()},
                                    target: "_blank",
                                    {result.result.name.clone()}
                                    }
                                    }
                                    td {
                                    {result.result.size.clone().replace(" ", "")}
                                    }
                                    //td {
                                    //{result.result.date.clone()}
                                    //}
                                }
                            }


                    }
                }
            }

        }
    }
        }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(app);
}
