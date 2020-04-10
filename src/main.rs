use bytes::buf::ext::BufExt;
use hyper::Client;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Leap");
    let versions = get_leap_versions().await?;
    let latest = get_latest_version(versions)
        .expect("Unable to get the latest version of the leap command.");
    println!("command latest version: {} {}", latest.name, latest.url);

    let versions = get_leap_project_template_versions().await?;
    let latest = get_latest_version(versions)
        .expect("Unable to get the latest version of the leap command.");
    println!(
        "project template latest version: {} {}",
        latest.name, latest.url
    );

    Ok(())
}

fn get_latest_version(tags: Vec<Tag>) -> Option<Tag> {
    match tags.first() {
        None => None,
        Some(tag) => Some(Tag {
            name: tag.name.clone(),
            url: tag.url.clone(),
        }),
    }
}

async fn get_leap_versions() -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url = get_github_tags_url("https://api.github.com/repos/daniel-samson/leap").await?;
    let tags = get_github_tags(tags_url).await?;
    Ok(get_versioned_tags(tags))
}

async fn get_leap_project_template_versions(
) -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url =
        get_github_tags_url("https://api.github.com/repos/daniel-samson/leap-project-template")
            .await?;
    let tags = get_github_tags(tags_url).await?;
    Ok(get_versioned_tags(tags))
}

async fn get_github_tags_url(
    api_url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let res = get_github(api_url).await?;
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

fn get_versioned_tags(tags: Vec<Value>) -> Vec<Tag> {
    tags.iter()
        .map(|tag| {
            (
                tag["name"].as_str().unwrap(),
                tag["zipball_url"].as_str().unwrap(),
            )
        })
        .filter(|(name, _)| name.starts_with("v"))
        .map(|(name, url)| Tag {
            name: String::from(name.trim_start_matches('v')),
            url: String::from(url),
        })
        .collect()
}

struct Tag {
    name: String,
    url: String,
}
