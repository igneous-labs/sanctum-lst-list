use std::error::Error;

use reqwest::header;
use sanctum_lst_list::{SanctumLst, SanctumLstList};
use tokio::task::JoinSet;

#[tokio::test]
async fn verify_all_token_logo_image_uri_valid() {
    let client: &'static reqwest::Client = Box::leak(Box::new(reqwest::Client::new()));
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
    let mut js = JoinSet::new();
    sanctum_lst_list.into_iter().for_each(|slst| {
        js.spawn(verify_token_logo_image_uri_valid(client, slst));
    });
    while let Some(res) = js.join_next().await {
        res.unwrap();
    }
}

async fn verify_token_logo_image_uri_valid(
    client: &reqwest::Client,
    SanctumLst {
        logo_uri, symbol, ..
    }: SanctumLst,
) {
    let content_type = match fetch_logo_image_uri_content_type(client, &logo_uri).await {
        Ok(ct) => ct,
        Err(e) => panic!("{symbol} fetch failed: {e}"),
    };
    assert!(
        content_type.to_lowercase().contains("image"),
        "{symbol} Content-Type {content_type} not image"
    );
}

async fn fetch_logo_image_uri_content_type(
    client: &reqwest::Client,
    logo_uri: &str,
) -> Result<String, Box<dyn Error>> {
    Ok(client
        .get(logo_uri)
        .send()
        .await?
        .error_for_status()?
        .headers()
        .get(header::CONTENT_TYPE)
        .ok_or("No Content-Type header")?
        .to_str()?
        .to_owned())
}
