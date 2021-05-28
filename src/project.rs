/*
 * Copyright IBM Corp. 2021, 2021
 *
 * This source code is licensed under the Apache 2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::error::DSMError;
use std::path::{Path, PathBuf};

pub struct Project {
    directory: PathBuf,
    package_json_path: PathBuf,
}

impl Project {
    pub fn find(cwd: PathBuf) -> Result<Project, DSMError> {
        let directories: Vec<&Path> = cwd
            .ancestors()
            .filter(|ancestor| ancestor.join("package.json").exists())
            .collect();
        let directory = directories.last().ok_or("")?.to_path_buf();
        let package_json_path = directory.join("package.json");

        Ok(Project {
            directory,
            package_json_path,
        })
    }
}

fn load_project_workspace() {}

fn load_child_workspace() {}

fn load_worktree() {}
