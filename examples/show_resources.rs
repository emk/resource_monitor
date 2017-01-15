//! This program will print out all the available resources known to the
//! `resource_monitor` crate.

#[macro_use]
extern crate error_chain;
extern crate resource_monitor;

use resource_monitor::{Resource, Result};

/// List all our resources.
fn run() -> Result<()> {
    let resources =
        &[Resource::Memory, Resource::OsMemory, Resource::AllocatorMemory];
    for res in resources {
        println!("{:?}:", res);
        println!("  limit: {:?}", res.limit()?);
        println!("  used: {:?}", res.used()?);
        println!("  available: {:?}", res.available()?);
    }
    Ok(())
}

/// Allow `error_chain` to declare a `main` function that calls `run`
/// and prints out any errors.  We basically do this so that so that
/// we can use `?` in `run`, because `?` only works in a function that
/// returns a `Result`, and `main` doesn't.
quick_main!(run);
