#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dioxus_sdk::utils::timing::{use_debounce, use_interval};
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};
use shared::File;
use std::env;
use std::time::Duration;
use async_std::task::sleep;
use std::rc::Rc;
//use dioxus_lazy::{lazy, List};

fn create_client() -> Client {
    let SEARCH_API_URL: &'static str = env!("SEARCH_API_URL");
    let SEARCH_API_KEY: &'static str = env!("SEARCH_API_KEY");
    Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap()
}

async fn execute_search(
    input: &str,
    page: &usize,
    client: &Client,
) -> meilisearch_sdk::search::SearchResults<File> {
    client
        .index("files")
        .search()
        .with_sort(&["platform.weight:desc"])
        .with_hits_per_page(40)
        .with_page(page.clone())
        .with_query(input)
        .execute::<File>()
        .await
        .unwrap()
    //.hits
}

fn app() -> Element {
    let mut input = use_signal(|| "".to_string());
    let mut page = use_signal(|| 1);
    //let mut footer = use_signal(|| None as Option<Rc<MountedData>>);
    let client = create_client();

    // after testing i like the instant results. Leaving this here for future optimising if needed.
    let mut debounce = use_debounce(Duration::from_millis(0), move |val| {
        page.set(1);
        input.set(val);
     });

    let results = use_resource(move || {
        to_owned![client];
        async move {
            if &input() == "" {
                return None;
            }
            
            Some(execute_search(&input(), &page(), &client).await)
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
                        style: "font-weight: 300;font-size: 0.8em;",
                        "Negative filters are available, e.g. '-nes'"
                }
            }
            input {
                r#type: "text",
                id: "search",
                name: "search",
                placeholder: "'street fighter snes' or 'street fighter -alpha -gba'",
                oninput: move |evt| {
                  debounce.action(evt.value());
                }
            }
            // search_input{}
           // List {
           // len: 100,
           // size: 400.,
          //  item_size: 20.,
          //  make_item: move |idx: &usize| rsx! { "Async item {*idx}" },
          //  make_value: lazy::from_async_fn(|idx| async move { idx })
        //}

            div {
                // style: "margin-top: 1em",
                style: "overflow-x:auto;",


                    table {
                        //style: "overflow-x:auto;",
                        tfoot {
                            //onmounted: move |element| footer.set(Some(element.data())),
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
                        //if let i = input.read() {
                        //  if i.is_empty() {
                        //    tr {
                        //      td {  colspan: 3, "how does it work: Search"}
                        //    }
                        //  }
                        //}
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

                            if r.total_pages.unwrap_or(0) > 1 {


                                tr {
                                    td {
                                        colspan: 3,
                                        div {
                                            style: "display: flex;flex-wrap: wrap;margin:0;",

                                        for n in 1..r.total_pages.unwrap_or(0) {
                                            div{
                                                style: "border:0;padding:3px 0px 4px 4px",
                                                a {
                                                    onclick: move |evt| {
                                                        page.set(n);
                                                      },
                                                    {n.to_string()}
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

        }
    }
        }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(app);
}

#[component]
fn search_input() -> Element {
    let mut count = use_signal(|| 0);

    //use_interval(Duration::from_millis(100), move || {
    //    count += 1;
    //});

    //let mut debounce = use_debounce(Duration::from_millis(2000), move |text| {
    //    println!("{text}");
    //    count.set(0);
    //});

    rsx! {
        p { "{count}" }
        button {
            onclick: move |_| {
                // Reset the counter after 2 seconds pass since the last click.
                //debounce.action("button was clicked");
            },
            "Reset the counter! (2 second debounce)"
        }
    }
}
