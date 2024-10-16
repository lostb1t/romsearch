use futures::{StreamExt, TryStreamExt};
use meilisearch_sdk::client::*;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use shared::{File, PlatformKind};
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use urlencoding::decode;

#[tokio::main]
async fn main() {
    dbg!("Started up");
    let _ = sync().await;
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
    // dbg!("Processing: ");
    // dbg!(&url);
    let mut files: Vec<File> = vec![];

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        //.with(RetryTransientMiddleware::new_with_policy(retry_policy))
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
    let date_selector = Selector::parse(".date").unwrap();
    drop(text);
    let mut futures = vec![];
    for element in fragment.select(&entry_selector) {
        let link_el = element.select(&link_selector).next().unwrap();
        let size_el = element.select(&size_selector).next().unwrap();
        let date_el = element.select(&date_selector).next().unwrap();
        let size = &size_el.text().collect::<Vec<_>>().first().unwrap().clone();
        let date = &date_el.text().collect::<Vec<_>>().first().unwrap().clone();
        let href = link_el.value().attr("href").unwrap().trim_matches('/');

        if *size.clone() == *"-" && *href != *".." {
            futures.push(parse_page(
                format!("{}/{}", url.clone(), href.clone()),
                search_client.clone(),
            ));
            //Box::pin(parse_page(
            //    format!(
            //        "{}/{}",
            //        url.clone(),
            //        href.clone()
            //    ),
            //    search_client.clone())).await;
        } else if *size.clone() != *"-" {
            let mut s = DefaultHasher::new();
            let name = href.clone();
            let location = format!("{}/{}", url.clone(), name);
            let platform_kind = PlatformKind::from_name(decode(&location).unwrap().to_string());

            let platform = match &platform_kind {
                Some(p) => shared::Platform::for_kind(p),
                None => None,
            };

            location.hash(&mut s);
            files.push(File {
                id: s.finish(),
                name: decode(href.clone()).unwrap().to_string(),
                location,
                size: Some(size.to_string()),
                date: Some(date.to_string()),
                // tags: vec![],
                platform: platform,
                // weight: match platform {
                //     Some(platform) => platform.weight,
                //     None => 0
                // }
            })
        }
    }
    drop(fragment);

    if !files.is_empty() {
        //dbg!("Commiting files");
        //dbg!(files.len());
        let _ = search_client
            .index("files")
            .add_or_update(&files, Some("id"))
            .await;
        // .unwrap()
        // //.wait_for_completion(&search_client, None, None)
        // .await
        // .unwrap();
    }

    drop(files);

    if !futures.is_empty() {
        //dbg!("Awaiting futures");
        //dbg!(futures.len());
        let mut stream = futures::stream::iter(futures).buffer_unordered(25);

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
    let search_client = Client::new(SEARCH_API_URL, Some(SEARCH_API_KEY)).unwrap();
    let searchable_attributes = ["name", "platform", "tags", "location"];
    let sortable_attributes = ["platform.weight"];
    //let platforms = Platform::platforms();
    let _ = search_client
        .index("files")
        .set_searchable_attributes(&searchable_attributes)
        .await;
    let _ = search_client
        .index("files")
        .set_sortable_attributes(&sortable_attributes)
        .await;

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_test() {
//         let entries: Vec<ParseResult> = vec![ParseResult {
//             file_name: "Tenet 2020 2160p UHD Webdl DTS-HD MA 5.1 x265-LEGi0N".to_string(),
//             year: Some(2020),
//             video_codec: Some(VideoCodecKind::H265),
//             video_resolution: Some(VideoResolutionKind::R2160P),
//             source: Some(VideoSourceKind::Webdl),
//             audio_codec: Some(AudioCodecKind::DTSHD),
//             ..ParseResult::default()
//         },
//         ParseResult {
//             file_name: "Tenet.2020.2160p.UHD.Webdl.dd5.1.x265-LEGi0N".to_string(),
//             year: Some(2020),
//             video_codec: Some(VideoCodecKind::H265),
//             video_resolution: Some(VideoResolutionKind::R2160P),
//             source: Some(VideoSourceKind::Webdl),
//             audio_codec: Some(AudioCodecKind::DD51),
//             ..ParseResult::default()
//         },
//         ParseResult {
//             file_name: "Sons.of.Anarchy.S03.720p.BluRay.CLUEREWARD".to_string(),
//             video_resolution: Some(VideoResolutionKind::R720P),
//             source: Some(VideoSourceKind::BluRay),
//             ..ParseResult::default()
//         }];
//         for entry in entries {
//             let result = parse(&entry.file_name);
//             assert_eq!(
//                 result,
//                 entry
//             );
//         }
//     }
// }
