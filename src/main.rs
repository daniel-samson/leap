use bytes::buf::ext::BufExt;
use directories::ProjectDirs;
use hyper::Client;
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use reqwest;
use reqwest::Response;
use semver::Version;
use serde_json::Value;
use std::fs::create_dir_all;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Leap is check for updates");
    let versions = get_leap_versions().await?;
    let latest_tag = get_latest_version(versions)
        .expect("Unable to get the latest version of the leap command.");
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))
        .expect("Unable to parse the current version of the leap command.");
    let latest_version = Version::parse(latest_tag.name.as_str())
        .expect("Unable to parse the latest version of the leap command.");
    if latest_version > current_version {
        println!(
            "A new version of the leap command is now available. Run cargo install leap to update."
        );
    }
    println!(
        "command latest version: {} {} {}",
        latest_tag.name, latest_tag.zipball, latest_tag.tarball
    );

    let versions = get_leap_project_template_versions().await?;
    let latest = get_latest_version(versions)
        .expect("Unable to get the latest version of the leap command.");
    println!(
        "project template latest version: {} {} {}",
        latest.name, latest.zipball, latest.tarball
    );

    let project_template_dirs = ProjectDirs::from("rs", "leap", "leap-project-template")
        .expect("Unable to get system directories");
    // TODO: Check if we have a cached version

    let template_dir = project_template_dirs.cache_dir().join(&latest.name);

    if !template_dir.exists() {
        create_dir_all(template_dir.as_path())?;
    }

    let template_path = template_dir.join("template.zip");
    if !&template_path.exists() {
        println!("Downloading {} from {}", &latest.name, &latest.zipball);
        let zipball = download_github(latest.zipball.as_str()).await?;
        let contents = zipball.bytes().await?;
        // TODO: Download latest version when there isn't a cached version available
        std::fs::write(&template_path, contents).unwrap();
    }

    // TODO: add logger to hide debug messages
    // TODO: Extract template into current working director
    println!("extracting...");
    extract_zip(template_path.as_path()).unwrap();
    let (short_sha, _) = latest.sha.split_at(7);
    let new_dir = format!("daniel-samson-leap-project-template-{}", short_sha);
    let project_name = "project-name";
    std::fs::rename(
        std::env::current_dir()?.join(new_dir.as_str()),
        std::env::current_dir()?.join(project_name),
    )
    .unwrap();
    println!("{}", new_dir);
    Ok(())
}

fn get_latest_version(tags: Vec<Tag>) -> Option<Tag> {
    match tags.first() {
        None => None,
        Some(tag) => Some(Tag {
            name: tag.name.clone(),
            zipball: tag.zipball.clone(),
            tarball: tag.tarball.clone(),
            sha: tag.sha.clone(),
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

async fn download_github(url: &str) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "https://leap.rs/")
        .send()
        .await?;
    Ok(response)
}

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

struct Tag {
    name: String,
    zipball: String,
    tarball: String,
    sha: String,
}

fn extract_zip(path: &Path) -> Result<(), String> {
    let file = std::fs::File::open(&path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}
