//! Config for the command line tool
//!

use directories::ProjectDirs;
use semver::Version;
use serde::{Deserialize, Serialize};

/// Get directory paths for cli
pub fn dir() -> Option<ProjectDirs> {
    directories::ProjectDirs::from("rs", "leap", "cli")
}

/// Get cli config or create a new config
pub fn config() -> Result<Config, std::io::Error> {
    if !exists() {
        return new();
    }
    read()
}

/// Does config exist?
fn exists() -> bool {
    if dir().is_none() {
        return false;
    }

    dir().unwrap().config_dir().exists()
}

/// Generate a new config for the cli
fn new() -> Result<Config, std::io::Error> {
    if !exists() {
        std::fs::create_dir_all(dir().unwrap().config_dir())?;
    }

    match write(Config::default()) {
        Ok(_) => Ok(Config::default()),
        Err(e) => Err(e),
    }
}

/// Read the config file
pub fn read() -> Result<Config, std::io::Error> {
    log::debug!("read config");
    if !exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot find config path".to_string(),
        ));
    }

    let toml_string = std::fs::read_to_string(dir().unwrap().config_dir().join("cli.toml"))?;
    let deserialize: Result<Config, toml::de::Error> = toml::from_str(toml_string.as_str());
    match deserialize {
        Ok(config) => Ok(config),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}

/// Update / write config file
pub fn write(config: Config) -> Result<(), std::io::Error> {
    let serialize: Result<String, toml::ser::Error> = toml::to_string(&config);
    match serialize {
        Ok(toml_string) => {
            match std::fs::write(dir().unwrap().config_dir().join("cli.toml"), toml_string) {
                Ok(_) => Ok(()),
                Err(e) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )),
            }
        }
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}

#[allow(dead_code)]
pub fn delete() -> Result<(), std::io::Error> {
    let p = dir().unwrap().config_dir().join("cli.toml");
    std::fs::remove_file(p)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageConfig {
    pub name: String,
    pub version: Version,
}

impl Default for PackageConfig {
    fn default() -> Self {
        PackageConfig {
            name: "leap".to_string(),
            version: Version::parse("0.2.0").unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateConfig {
    pub hash: String,
    pub version: Version,
    pub compressed: String,
    pub extracted: String,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        TemplateConfig {
            hash: "6cdba5e".to_string(),
            version: Version::parse("0.2.0").unwrap(),
            compressed: "template-6cdba5e.zip".to_string(),
            extracted: "daniel-samson-leap-project-template-6cdba5e".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateConfig {
    pub last: usize,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        UpdateConfig { last: 1 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub package: PackageConfig,
    pub template: TemplateConfig,
    pub update: UpdateConfig,
}

impl Config {
    pub fn with_package(&self, package: PackageConfig) -> Config {
        Config {
            package,
            template: self.template.clone(),
            update: self.update.clone(),
        }
    }

    pub fn with_template(&self, template: TemplateConfig) -> Config {
        Config {
            package: self.package.clone(),
            template,
            update: self.update.clone(),
        }
    }

    pub fn with_update(&self, update: UpdateConfig) -> Config {
        Config {
            package: self.package.clone(),
            template: self.template.clone(),
            update,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            package: PackageConfig::default(),
            template: TemplateConfig::default(),
            update: UpdateConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_works() {
        assert!(delete().is_ok());
        let new_config = new();
        assert!(new_config.is_ok());
        let config = read();
        assert!(config.is_ok());
        assert_eq!(
            config.unwrap().template.hash,
            new_config.unwrap().template.hash
        );
    }
}
