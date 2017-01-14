//! # `resource_monitor`: Check resources available to the current process
//!
//! Right now, we only support checking the available RAM on a Linux system
//! using cgroups. This should work inside a Docker container, or outside
//! of a container on at least Ubuntu 16.04.
//!
//! ```
//! let res = resource_monitor::Resource::Memory;
//! println!("Memory:");
//! println!("  limit: {}", res.limit().unwrap());
//! println!("  used: {}", res.used().unwrap());
//! println!("  available: {}", res.available().unwrap());
//! ```
//!
//! Patches to add new resource types and new kinds of limits (`getrlimit`,
//! etc.) are very much welcome! In particular, if submitting a PR, please
//! be careful to explain how the different limits interact.

#![warn(missing_docs)]

// Needed for `error_chain`, which does evil things with macros.
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

use std::fs;
use std::io::prelude::*;
use std::path::Path;

// Re-export our error types declared by `error-chain`.
pub use errors::{Error, ErrorKind, Result};
use errors::ResultExt;

/// Rust's standard error-handling boilerplate is obnoxious, so we use
/// the `[error-chain][]` crate's `error_chain!` macro to declare a new error
/// type with all the bells and whistles. This is a semi-official crate
/// by brson, a Rust core team member.
///
/// [error-chain]: https://docs.rs/error-chain/0.7.2/error_chain/
mod errors {
    use std::path::PathBuf;

    error_chain! {
        errors {
            /// An error occurred while trying to access the specified path.
            File(path: PathBuf) {
                // This is used when Rust wants to display this error
                // without allocating memory.
                description("could not access file with limit data")
                // This is used to display the error to the user. Note
                // that we have to call `.display()` on path objects to
                // get something that's valid, printable UTF-8.
                display("could not access {}", path.display())
            }
        }
    }
}

/// Read a file containing an integer.
fn read_file_u64(path: &Path) -> Result<u64> {
    // Declare a helper function to create an error wrapper containing
    // the path we were trying to read, or our callers will hate us.
    let mkerr = || ErrorKind::File(path.to_owned());

    // Read a number out of the specified file and parse it.  We have
    // to allocate a string because Rust is allergic to allocating strings
    // in APIs we might use from inside loops.  The `?` operator checks
    // for an error and `return`s immediately if it finds one.
    let mut s = String::new();
    let mut f: fs::File = fs::File::open(path).chain_err(&mkerr)?;
    f.read_to_string(&mut s).chain_err(&mkerr)?;
    s.trim().parse().chain_err(&mkerr)
}

/// Types of resource we can monitor.  This type may be extended with
/// new variants; do not attempt to exhaustively match against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    /// RAM in bytes.
    Memory,
    /// A private internal variant to allow future extensibility.
    #[doc(hidden)]
    __Private,
}

impl Resource {
    /// What is the maximum amount of the resource this process may consume?
    pub fn limit(&self) -> Result<u64> {
        match *self {
            Resource::Memory => {
                let path = "/sys/fs/cgroup/memory/memory.limit_in_bytes";
                read_file_u64(&Path::new(path))
            }
            Resource::__Private => {
                unreachable!("Do not use Resource::__Private")
            }
        }
    }

    /// What is the current amount of the resource consumed by this process?
    pub fn used(&self) -> Result<u64> {
        match *self {
            Resource::Memory => {
                let path = "/sys/fs/cgroup/memory/memory.usage_in_bytes";
                read_file_u64(&Path::new(path))
            }
            Resource::__Private => {
                unreachable!("Do not use Resource::__Private")
            }
        }
    }

    /// How much of the resource is available to the process but not yet used?
    pub fn available(&self) -> Result<u64> {
        let l = self.limit()?;
        let u = self.used()?;
        if u > l { Ok(0) } else { Ok(l - u) }
    }
}
