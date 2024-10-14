use std::env;
use futures::{StreamExt, TryStreamExt};
use meilisearch_sdk::client::*;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use urlencoding::decode;

#[tokio::main]
async fn main() {
    dbg!("Started up");
    let _ = sync().await;
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    id: u64,
    name: String,
    location: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Retry;
impl RetryableStrategy for Retry {
    fn handle(
        &self,
        res: &std::result::Result<reqwest::Response, reqwest_middleware::Error>,
    ) -> Option<Retryable> {
        match res {
            Ok(success) => None,
            Err(error) => {
                // get a channelClosed error sometimes. Just force that shit.
                if error.status().is_none() {
                    dbg!("RETRY");
                    return Some(Retryable::Transient);
                };
                default_on_request_failure(error)
            }
        }
    }
}

async fn parse_page(url: String, search_client: Client) -> Result<(), anyhow::Error> {
    dbg!("Processing: ");
    dbg!(&url);
    let mut files: Vec<File> = vec![];

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .with(RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            Retry,
        ))
        .build();

    //let text = client.get(&url).send().await?.text().await?;

    let text = match client.get(&url).send().await {
        Ok(res) => res.text().await?,
        Err(e) => {
            dbg!(&e);
            return Ok(());
        }
    };

    let fragment = Html::parse_fragment(text.as_str());
    let entry_selector = Selector::parse("#list tbody tr").unwrap();
    let link_selector = Selector::parse(".link a").unwrap();
    let size_selector = Selector::parse(".size").unwrap();

    let mut futures = vec![];
    for element in fragment.select(&entry_selector) {
        let link_el = element.select(&link_selector).next().unwrap();
        let size_el = element.select(&size_selector).next().unwrap();
        let size = &size_el.text().collect::<Vec<_>>().first().unwrap().clone();
        let href = link_el.value().attr("href").unwrap().trim_matches('/');

        if *size.clone() == *"-" && *href != *"../" {
            futures.push(parse_page(
                format!(
                    "{}/{}",
                    url.clone(),
                    href.clone()
                ),
                search_client.clone(),
            ));
        } else if *size.clone() != *"-" {
            let mut s = DefaultHasher::new();
            // we got a files
            let name = href.clone();
            let location = format!("{}/{}", url.clone(), name);
            location.hash(&mut s);
            files.push(File {
                id: s.finish(),
                name: decode(href.clone()).unwrap().to_string(),
                location,
            })
        }
    }

    if !files.is_empty() {
        dbg!("Commiting files");
        dbg!(files.len());
        let _ = search_client
            .index("files")
            .add_or_update(&files, Some("id"))
            .await
            .unwrap()
            .wait_for_completion(&search_client, None, None)
            .await
            .unwrap(); 
    }
    
    files = vec![];

    if !futures.is_empty() {
        dbg!("Awaiting futures");
        dbg!(futures.len());
        let mut stream = futures::stream::iter(futures).buffer_unordered(10);

        while let Some(response) = stream.next().await {
            // handle response
        }
    }

    Ok(())
    // files.append(&mut stream);

    // Ok(files)
}

async fn sync() -> Result<(), anyhow::Error> {
    let SEARCH_API_URL = env::var("SEARCH_API_URL").unwrap();
    let SEARCH_API_KEY = env::var("SEARCH_API_KEY").unwrap();
    let search_client = Client::new(
        SEARCH_API_URL,
        Some(SEARCH_API_KEY),
    )
    .unwrap();

    parse_page("https://myrient.erista.me/files".to_string(), search_client)
        .await
        .unwrap();

    // dbg!("TOTAL FILES: ");
    // dbg!(&files.len());

    // let total = files.len();
    // let mut chunk_size = 2500;

    // if total < chunk_size {
    //     chunk_size = total;
    // }
    // let mut chunks = total / chunk_size;
    // let mut i = 1;
    // for chunk in files.chunks(chunks) {
    //     dbg!("processing chunk: ");
    //     dbg!(i);
    //     i += 1;
    //     //dbg!(chunk_size);
    //     let _ = client
    //         .index("files")
    //         .add_or_update(chunk, Some("id"))
    //         .await
    //         .unwrap()
    //         .wait_for_completion(&client, None, None)
    //         .await
    //         .unwrap();
    // }
    Ok(())
}
