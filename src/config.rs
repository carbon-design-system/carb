use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("Unable to find config from: {0}")]
    DoesNotExist(String),

    #[error("Unable to deserialize config")]
    Deserialize { source: serde_yaml::Error },

    #[error("Unable to serialize config")]
    Serialize { source: serde_yaml::Error },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    version: Version,
}

impl Config {
    /// Load the configuration from the given directory
    pub fn load<P>(directory: P) -> Result<Config, ConfigError>
    where
        P: AsRef<Path>,
    {
        let config_path = Config::exists(directory)?;
        let contents = fs::read_to_string(config_path)?;
        let config: Config = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(err) => return Err(ConfigError::Deserialize { source: err }),
        };

        Ok(config)
    }

    /// Save the configuration to a predetermined location in the given directory
    pub fn save<P>(&self, directory: P) -> Result<(), ConfigError>
    where
        P: AsRef<Path>,
    {
        let contents = match serde_yaml::to_string(self) {
            Ok(contents) => contents,
            Err(err) => return Err(ConfigError::Serialize { source: err }),
        };

        let config_path = Config::get_config_path(&directory);
        let config_dir = config_path
            .parent()
            .expect("Expected config to be created in a directory");

        println!("{:?}", config_path);
        fs::create_dir_all(config_dir)?;
        fs::write(config_path, contents)?;

        Ok(())
    }

    // Get the config path from a given directory
    fn get_config_path<P>(directory: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        directory.as_ref().join(".carb").join("config.yml")
    }

    // Check to see if the configuration exists from the given directory
    fn exists<P>(directory: P) -> Result<PathBuf, ConfigError>
    where
        P: AsRef<Path>,
    {
        let config_path = directory.as_ref().join(".carb").join("config.yml");

        if config_path.exists() {
            Ok(config_path)
        } else {
            Err(ConfigError::DoesNotExist(format!(
                "{}",
                config_path.display()
            )))
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: Version::Prerelease,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Version {
    #[serde(rename = "prerelease")]
    Prerelease,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn it_should_load_config() -> Result<()> {
        let directory = tempdir()?;
        let config = Config::default();

        config.save(directory.path())?;

        let result = Config::load(directory.path())?;

        assert_eq!(config, result);

        Ok(())
    }

    #[test]
    fn it_should_save_config_in_file() -> Result<()> {
        let directory = tempdir()?;
        let config = Config::default();

        config.save(directory.path())?;

        assert!(directory.path().join(".carb").join("config.yml").exists());

        Ok(())
    }
}
