use bytes::buf::ext::BufExt;
use hyper::Client;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Getting tags_url");
    let res = get_github("https://api.github.com/repos/daniel-samson/leap").await?;
    let tags = match res["tags_url"].as_str() {
        None => panic!("failed to get tags_url"),
        Some(tags_url) => {
            println!("Getting tags");
            let res = get_github(tags_url).await?;
            let tags = res.as_array().cloned();
            tags
        }
    };
    match tags {
        None => panic!("failed to get tags"),
        Some(tags) => {
            for tag in tags.into_iter() {
                let name = tag["name"].as_str().unwrap();
                let download_url = tag["zipball_url"].as_str().unwrap();
                println!("{}: {}", name, download_url);
            }
        }
    }

    Ok(())
}

async fn get_github(url: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "https://leap.rs/")
        .body(Body::from(r#""#))
        .unwrap();

    // Await the response...
    let res = client.request(req).await.unwrap();
    let body = hyper::body::aggregate(res).await.unwrap();
    let reader = body.reader();
    let v: Value = serde_json::from_reader(reader).unwrap();
    Ok(v)
}
