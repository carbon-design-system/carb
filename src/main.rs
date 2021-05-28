/*
 * Copyright IBM Corp. 2021, 2021
 *
 * This source code is licensed under the Apache 2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod cli;
mod error;
mod project;
mod workspaces;

use std::env;
// Note: included to make sure `from_args` is brought into scope for the given trait. From the
// help for this error:
// help: items from traits can only be used if the trait is in scope
use structopt::StructOpt;

fn main() {
    let args = cli::CLI::from_args();

    match args {
        cli::CLI::Workspaces(cli::Workspaces::List {}) => {
            let path = env::current_dir().expect("Expected `dsm` to be ran in a directory");
            let project = project::Project::find(path)?;

            workspaces::list();
        }
        cli::CLI::Cache(cli::Cache::Clean {}) => {
            println!("Clean cache");
        }
    }
}

// mod error;
// mod project;

// use project::Project;
// use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::PathBuf;
// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// #[structopt(name = "dsm")]
// enum CLI {
// Release {},
// Run {},
// Version {},
// }

// fn main() {
// let path = env::current_dir().expect("cwd");
// let project = Project::find(path).expect("");

// Ok(())

// let package_json_paths: Vec<PathBuf> = path
// .ancestors()
// .filter_map(|ancestor| {
// let package_json_path = ancestor.join("package.json");
// if package_json_path.exists() {
// Some(package_json_path)
// } else {
// None
// }
// })
// .collect();
// let package_json_path = package_json_paths.last();

// match package_json_path {
// Some(package_json_path) => {
// println!("FOUND {:?}", package_json_path);

// let file = File::open(package_json_path).expect("package.json should read");
// let reader = BufReader::new(file);
// let json: serde_json::Value =
// serde_json::de::from_reader(reader).expect("package.json should deserialize");

// match json["namez"] {
// serde_json::Value::Null => println!("DOES NOT EXIST"),
// _ => println!("GOOD TO GO BABY"),
// }
// }
// None => println!("NONE"),
// }

// let project = Project::find();

// println!("{}", project);

// let args = CLI::from_args();
// println!("{:?}", args);

// match args {
// CLI::Run {} => println!("Run"),
// _ => println!("HELPPP"),
// };
// }
