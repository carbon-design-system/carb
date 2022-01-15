use std::collections::HashMap;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Project {
    pub directory: PathBuf,
    pub workspace: Workspace,
}

impl Project {
    pub fn find<P: AsRef<Path>>(cwd: P) -> Result<Self> {
        let directory = cwd.as_ref();
        let paths: Vec<&Path> = directory
            .ancestors()
            .filter(|ancestor| ancestor.join("package.json").exists())
            .collect();
        let root = paths.last();
        let workspace = root
            .map(Workspace::load)
            .transpose()?
            .ok_or("No workspace available")?;
        let project = Project {
            directory: root.unwrap().into(),
            workspace,
        };

        Ok(project)
    }
}

#[derive(Debug)]
pub struct Workspace {
    pub directory: PathBuf,
    pub package_json: PackageJson,
    pub workspaces: Vec<Workspace>,
}

impl Workspace {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let directory = path.as_ref();
        let contents = std::fs::read_to_string(directory.join("package.json"))?;
        let package_json: PackageJson = serde_json::from_str(&contents)?;
        let mut workspaces = Vec::new();

        if let Some(ref patterns) = package_json.workspaces {
            let walker = globwalk::GlobWalkerBuilder::from_patterns(directory, patterns)
                .build()?
                .into_iter()
                .filter_map(std::result::Result::ok);

            for item in walker {
                workspaces.push(Workspace::load(item.path())?);
            }
        }

        Ok(Self {
            directory: directory.into(),
            package_json,
            workspaces,
        })
    }

    // pub fn run_command(&self, command: String) -> Result<()> {
    // }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspaces: Option<Vec<String>>,
    scripts: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    impl PackageJson {
        fn new<T: Into<String>>(name: T, version: T) -> Self {
            PackageJson {
                name: name.into(),
                version: version.into(),
                workspaces: None,
                scripts: Default::default(),
            }
        }

        fn with_workspaces(self, workspaces: Vec<String>) -> Self {
            Self {
                workspaces: Some(workspaces),
                ..self
            }
        }

        fn to_string(&self) -> serde_json::Result<String> {
            serde_json::to_string(&self)
        }
    }

    #[test]
    fn should_work() -> Result<()> {
        let directory = tempdir()?;

        std::fs::create_dir(directory.path().join("project"))?;
        std::fs::create_dir(directory.path().join("project/packages"))?;
        std::fs::create_dir(directory.path().join("project/packages/components"))?;
        std::fs::create_dir(directory.path().join("project/packages/react"))?;

        std::fs::write(
            directory.path().join("project/package.json"),
            PackageJson::new("root", "0.0.0")
                .with_workspaces(vec!["packages/*".to_string()])
                .to_string()?,
        )?;

        std::fs::write(
            directory
                .path()
                .join("project/packages/components/package.json"),
            PackageJson::new("components", "1.0.0").to_string()?,
        )?;

        std::fs::write(
            directory.path().join("project/packages/react/package.json"),
            PackageJson::new("react", "2.0.0").to_string()?,
        )?;

        let project = Project::find(directory.path().join("project/packages/react"))?;

        println!("{:?}", project);

        Ok(())
    }
}
