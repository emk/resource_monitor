//! This program will attempt to use most available memory.

#[macro_use]
extern crate error_chain;
extern crate resource_monitor;

use resource_monitor::{Resource, Result};

fn run() -> Result<()> {
    let mut used_memory = vec![];
    loop {
        let avail = Resource::Memory.available()?;
        println!("Available: {}", avail);
        if avail < 10_000_000 {
            break;
        }

        // Try filling memory with small chunks.
        for _ in 0..10_000 {
            used_memory.push(vec![0u8; 100]);
        }

        // ..or large chunks.
        //used_memory.push(vec![0u8; 1_000_000]);
    }
    println!("Clearing");
    drop(used_memory);
    println!("Available: {}", Resource::Memory.available()?);
    Ok(())
}

/// Allow `error_chain` to declare a `main` function that calls `run`
/// and prints out any errors.  We basically do this so that so that
/// we can use `?` in `run`, because `?` only works in a function that
/// returns a `Result`, and `main` doesn't.
quick_main!(run);
