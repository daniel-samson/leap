use bytes::buf::ext::BufExt;
use hyper::Client;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Versions:");
    let tags_url = get_github_tags_url().await?;
    let tags = get_github_tags(tags_url).await?;
    let versions = get_versions_from_tags(tags);

    for version in versions {
        println!("{}: {}", version.name, version.url);
    }

    Ok(())
}

async fn get_github_tags_url() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let res = get_github("https://api.github.com/repos/daniel-samson/leap").await?;
    let tags_url = res["tags_url"].as_str().expect("failed to get tags_url");
    Ok(String::from(tags_url))
}

async fn get_github_tags(
    tags_url: String,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let res = get_github(tags_url.as_str()).await?;
    let tags = res.as_array().expect("failed to parse tags");
    Ok(tags.clone())
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

fn get_versions_from_tags(tags: Vec<Value>) -> Vec<Version> {
    tags.iter()
        .map(|tag| {
            (
                tag["name"].as_str().unwrap(),
                tag["zipball_url"].as_str().unwrap(),
            )
        })
        .filter(|(name, _)| name.starts_with("v"))
        .map(|(name, url)| Version {
            name: String::from(name.trim_start_matches('v')),
            url: String::from(url),
        })
        .collect()
}

struct Version {
    name: String,
    url: String,
}
