use clap::{App, AppSettings, Arg, SubCommand};
use directories::ProjectDirs;
use log::info;
use reqwest;
use reqwest::Response;
use semver::Version;
use serde_json::Value;
use std::fs::create_dir_all;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let matches = App::new("leap")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(clap::crate_version!())
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Provides the tooling you need to work with the leap framework")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new project")
                .version("0.1.0")
                .arg(
                    Arg::with_name("NAME")
                        .help("The name you wish to call your project")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(command) = matches.subcommand_matches("new") {
        new_project(command.value_of("NAME").expect("Missing NAME")).await?;
    }

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

async fn new_project(project_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("leap is checking for updates ...");
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

    info!(
        "command latest version: {} {} {}",
        latest_tag.name, latest_tag.zipball, latest_tag.tarball
    );

    let versions = get_leap_project_template_versions().await?;
    let latest = get_latest_version(versions)
        .expect("Unable to get the latest version of the leap command.");
    info!(
        "project template latest version: {} {} {}",
        latest.name, latest.zipball, latest.tarball
    );

    let project_template_dirs = ProjectDirs::from("rs", "leap", "leap-project-template")
        .expect("Unable to get system directories");

    let template_dir = project_template_dirs.cache_dir().join(&latest.name);

    if !template_dir.exists() {
        create_dir_all(template_dir.as_path())?;
    }

    let template_path = template_dir.join("template.zip");
    if !&template_path.exists() {
        info!("Downloading {} from {}", &latest.name, &latest.zipball);
        let zipball = download_github(latest.zipball.as_str()).await?;
        let contents = zipball.bytes().await?;
        std::fs::write(&template_path, contents).unwrap();
    }

    info!("extracting template...");
    extract_zip(template_path.as_path(), project_template_dirs.data_dir()).unwrap();
    let (short_sha, _) = latest.sha.split_at(7);
    let new_dir = format!("daniel-samson-leap-project-template-{}", short_sha);

    if project_template_dirs
        .data_dir()
        .join(new_dir.as_str())
        .exists()
    {
        info!("renaming template...");
        let action = std::fs::rename(
            project_template_dirs.data_dir().join(new_dir.as_str()),
            std::env::current_dir()?.join(project_name),
        );

        info!("copying template...");
        match action {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to create project because the project name already exists");
            }
        }
    }

    Ok(())
}

async fn get_leap_versions() -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url = get_github_tags_url("https://daniel-sanson:727dbf8f4b0476acfd530e80cf570e6f2149e4fd@api.github.com/repos/daniel-samson/leap").await?;
    let tags = get_github_tags(tags_url).await?;
    Ok(get_versioned_tags(tags))
}

async fn get_leap_project_template_versions(
) -> Result<Vec<Tag>, Box<dyn std::error::Error + Send + Sync>> {
    let tags_url =
        get_github_tags_url("https://daniel-sanson:727dbf8f4b0476acfd530e80cf570e6f2149e4fd@api.github.com/repos/daniel-samson/leap-project-template")
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
    let response = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "https://leap.rs/")
        .send()
        .await?;
    info!("status: {}", response.status());
    let body = response.text().await?;
    info!("body: {}", &body);
    let v: Value = serde_json::from_str(body.as_str()).unwrap();
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

fn extract_zip(path: &Path, extract_to: &Path) -> Result<(), String> {
    let file = std::fs::File::open(&path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = extract_to.join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                info!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            info!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            info!(
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
