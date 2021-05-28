/*
 * Copyright IBM Corp. 2021, 2021
 *
 * This source code is licensed under the Apache 2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Reference for building up internal error types:
// https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/fqlmknt/
#[derive(Debug)]
pub enum DSMError {
    /// Represents all other cases of `std::io::Error`
    IOError(std::io::Error),
}

impl std::error::Error for DSMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DSMError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for DSMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DSMError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for DSMError {
    fn from(err: std::io::Error) -> DSMError {
        DSMError::IOError(err)
    }
}
