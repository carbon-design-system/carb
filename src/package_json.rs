use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
    pub name: Option<String>,
    pub private: Option<bool>,
    pub version: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub workspaces: Option<Vec<String>>,

    #[serde(rename = "dependencies")]
    pub dependencies: Option<HashMap<String, String>>,

    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,

    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<HashMap<String, String>>,
}

impl PackageJson {
    /// Get the relevant fields from the contents of a package.json file
    pub fn from_package_json(contents: &str) -> Result<PackageJson, serde_json::Error> {
        let package_json: PackageJson = serde_json::from_str(&contents)?;
        Ok(package_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn parse_package_json() -> Result<()> {
        let data = r#"
        {
            "name": "test",
            "version": "0.1.0",
            "scripts": {
                "build": "rollup"
            },
            "peerDependencies": {
                "react": "^1.0.0"
            },
            "dependencies": {
                "classnames": "^1.0.0"
            },
            "devDependencies": {
                "rollup": "^1.0.0"
            }
        }
        "#;
        let package_json = PackageJson::from_package_json(data)?;

        assert_eq!(package_json.name, Some("test".into()));

        Ok(())
    }

    #[test]
    fn supports_extra_fields() -> Result<()> {
        let data = r#"
        {
            "name": "test",
            "extra-field": true
        }
        "#;
        let package_json = PackageJson::from_package_json(data)?;

        assert_eq!(package_json.name, Some("test".into()));

        Ok(())
    }
}
