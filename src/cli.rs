/*
 * Copyright IBM Corp. 2021, 2021
 *
 * This source code is licensed under the Apache 2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dsm", about = "A CLI for managing design systems")]
pub enum CLI {
    /// Interact with the cache for `dsm`
    Cache(Cache),

    /// Explore the workspaces in your project
    Workspaces(Workspaces),
}

#[derive(Debug, StructOpt)]
pub enum Cache {
    /// Clean the cache for `dsm`
    Clean {},
}

#[derive(Debug, StructOpt)]
pub enum Workspaces {
    /// List the workspaces in your project
    List {},
}
