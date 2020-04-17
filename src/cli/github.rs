use async_std::task;
use serde_json::Value;

pub struct Tag {
    pub name: String,
    pub zipball: String,
    pub tarball: String,
    pub sha: String,
}

/// get the version for the leap repository
#[allow(dead_code)]
#[allow(unused)]
pub fn get_leap_versions() -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url = get_tags_url("https://api.github.com/repos/daniel-samson/leap")?;
    let tags = get_tags(tags_url)?;
    Ok(get_versioned_tags(tags))
}

/// get the versions fro the leap project repository
pub fn get_leap_project_template_versions(
) -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url =
        get_tags_url("https://api.github.com/repos/daniel-samson/leap-project-template")?;
    let tags = get_tags(tags_url)?;
    Ok(get_versioned_tags(tags))
}

/// Get the tags url for a given repository
fn get_tags_url(api_url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let res = get(api_url)?;
    let tags_url = res["tags_url"].as_str().expect("failed to get tags_url");
    Ok(String::from(tags_url))
}

/// Get the tags available
fn get_tags(tags_url: String) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let res = get(tags_url.as_str())?;
    let tags = res.as_array().expect("failed to parse tags");
    Ok(tags.clone())
}

/// Get a list of tags which look like versions
fn get_versioned_tags(tags: Vec<Value>) -> Vec<Tag> {
    tags.iter()
        .map(|tag| {
            (
                tag["name"].as_str().unwrap(),
                tag["zipball_url"].as_str().unwrap(),
                tag["tarball_url"].as_str().unwrap(),
                tag["commit"]["sha"].as_str().unwrap(),
            )
        })
        .filter(|(name, _, _, _)| name.starts_with("v"))
        .map(|(name, zipball, tarball, sha)| Tag {
            name: String::from(name.trim_start_matches('v')),
            zipball: String::from(zipball),
            tarball: String::from(tarball),
            sha: String::from(sha),
        })
        .collect()
}

/// Make a GET request
fn get(url: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    task::block_on(async {
        let response = surf::get(url)
            .set_header("Accept", "application/vnd.github.v3+json")
            .set_header("User-Agent", "https://leap.rs/")
            .recv_string()
            .await?;
        log::info!("body: {}", &response);
        let v: Value = serde_json::from_str(response.as_str()).unwrap();
        Ok(v)
    })
}
