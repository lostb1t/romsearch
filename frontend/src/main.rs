#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};
use shared::File;
use std::env;

fn create_client() -> Client {
    let SEARCH_API_URL: &'static str = env!("SEARCH_API_URL");
    let SEARCH_API_KEY: &'static str = env!("SEARCH_API_KEY");
    Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap()
}

async fn execute_search(
    input: &str,
    client: &Client,
) -> meilisearch_sdk::search::SearchResults<File> {
    client
        .index("files")
        .search()
        .with_sort(&["platform.weight:desc"])
        .with_limit(40)
        .with_query(input)
        .execute::<File>()
        .await
        .unwrap()
    //.hits
}

fn app() -> Element {
    let mut input = use_signal(|| "".to_string());
    let client = create_client();

    let results = use_resource(move || {
        to_owned![client];
        async move {
            if &input() == "" {
                return None;
            }
            Some(execute_search(&input(), &client).await)
        }
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

            div { 
                // style: "display: grid;grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));", 
                style: "display: flex;flex-wrap: wrap;padding-bottom:20px;padding-top:20px;",
                div { 
                    style: "", 
                    h1 { style: "padding:0;padding-right: 5px", "Game rom search /" } 
                } 
                div { 
                    // h1 { 
                        style: "font-weight: 300;font-size: 0.8em;", 
                        "Console filters are available, e.g. 'NES'."
                        // "add a console abbr to search by console, ex: street fighter nes"
                    // } 
                }
            }
            input {
                r#type: "text",
                id: "search",
                name: "search",
                placeholder: "Search a something...",
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
                                    colspan: 3,
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
                            th { "platform" }
                            th { "size" }
                        }
                        if let Some(Some(r)) = results.read().as_ref() {
                        if r.hits.is_empty() {
                            tr {
                              td {  colspan: 3, "No results"}
                            }
                        } else {
                            for result in r.hits.iter() {
                                tr {
                                    td {
                                    a {
                                    href: {result.result.location.clone()},
                                    target: "_blank",
                                    {result.result.name.clone()}
                                    }
                                    }

                                    td {

                                      if result.result.platform.is_some() {
                                       {result.result.platform.clone().unwrap().kind.to_string()}
                                      } else {
                                        "-"
                                      }
                                    }
                                    td {
                                    {result.result.size.clone().unwrap_or("-".to_string()).replace(" ", "")}
                                    }

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
