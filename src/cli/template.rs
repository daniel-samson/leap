//! Template manager

use crate::cli::{config, download, github};

use crate::cli::config::{TemplateConfig, UpdateConfig};
use crate::cli::zip;
use semver::Version;

pub fn update() {
    let config = config::config();
    if config.is_err() {
        log::error!(
            "Unable to get config - {}",
            config.err().unwrap().to_string()
        );
        return;
    }

    let config = config.unwrap();
    let dir = config::dir().unwrap();
    let now: usize = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("The day the earth stood still?")
        .as_secs() as usize;

    let last_six_months = now - 15_778_800_usize;
    let its_been_ages = last_six_months < config.update.last;

    // todo: check &config.update.last
    if its_been_ages || !dir.data_dir().join(&config.template.compressed).exists() {
        match github::get_leap_project_template_versions() {
            Ok(versions) => {
                let latest_tags = versions.first().unwrap();
                let url = &latest_tags.zipball;
                // Update Config
                let (short_sha, _) = &latest_tags.sha.split_at(7);
                let version = Version::parse(&latest_tags.name).unwrap();

                let config = config.with_template(TemplateConfig {
                    hash: format!(r"template-{}.zip", short_sha),
                    version,
                    compressed: format!(r"template-{}.zip", short_sha),
                    extracted: format!(r"daniel-samson-leap-project-template-{}", short_sha),
                });

                let config = config.with_update(UpdateConfig { last: now });

                // Persist Config
                match config::write(config.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("unable to update config because {}", e.to_string());
                        return;
                    }
                };
                // Download
                match download::get(url) {
                    Ok(data) => {
                        if data.is_empty() {
                            log::error!("the project template is empty");
                            return;
                        };

                        match std::fs::create_dir_all(&dir.data_dir()) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!(
                                    "unable to create cache project template because {}",
                                    e.to_string()
                                );
                                return;
                            }
                        };

                        match std::fs::write(
                            &dir.data_dir().join(&config.template.compressed),
                            data,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!(
                                    "unable to cache project template because {}",
                                    e.to_string()
                                );
                                return;
                            }
                        };
                    }
                    Err(e) => {
                        log::error!(
                            "unable to get the latest version of the project template because {}",
                            e.as_ref()
                        );
                        return;
                    }
                };
            }
            Err(e) => {
                log::error!(
                    "unable to get the latest version of the project template because {}",
                    e.as_ref()
                );
                return;
            }
        };
    }

    if !dir.data_dir().join(&config.template.extracted).exists() {
        match zip::extract(
            &dir.data_dir().join(&config.template.compressed),
            &dir.data_dir().join(&config.template.extracted),
        ) {
            Ok(_) => {}
            Err(e) => {
                log::error!(
                    "unable to extract project template because {}",
                    e
                );
                return;
            }
        }
    }
}

pub fn new_project(name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut config = config::config()?;
    let dir = config::dir().unwrap();
    let template_path = dir.data_dir()
        .join(&config.template.extracted)
        .join(&config.template.extracted);

    if !template_path.exists() {
        update();
        // reload config to get latest path names
        config = config::config()?;
    }

    let template_path = dir.data_dir()
        .join(&config.template.extracted)
        .join(&config.template.extracted);

    log::info!("renaming template...");
    let action = std::fs::rename(
        &template_path,
        std::env::current_dir()?.join(name),
    );

    log::info!("copying template...");
    match action {
        Ok(_) => {}
        Err(_) => {
            println!("Unable to create project because the project name already exists");
        }
    }

    Ok(())
}
