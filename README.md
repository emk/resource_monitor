# `resource_monitor`: Check resources available to the current process

Right now, we only support checking the available RAM on a Linux system
using cgroups. This should work inside a Docker container, or outside
of a container on at least Ubuntu 16.04.

```rust
let res = resource_monitor::Resource::Memory;
println!("Memory:");
println!("  limit: {}", res.limit().unwrap());
println!("  used: {}", res.used().unwrap());
println!("  available: {}", res.available().unwrap());
```

**TODO:** For most allocators, this crate will only show you a high-water
mark, and not the currently available RAM. I'm looking at how to hook it
into the `jemalloc` stats interface so I can report combined available
system and allocator RAM.

Patches to add new resource types and new kinds of limits (`getrlimit`,
etc.) are very much welcome! In particular, if submitting a PR, please
be careful to explain how the different kinds of OS limits interact, and
which limit a given process will hit first. If you make me look this up,
I may procrastinate on merging, sadly. :-)

## Building

You'll need a Rust toolchain.  See [the Rust homepage][Rust] for download
instructions, or if you trust SSL, run:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Make sure `~/.cargo/bin` is in your path.

To build:

```sh
git clone https://github.com/emk/resource_monitor.git
cd resource_monitor
cargo build
```

## Reading the code

Start with:

- [examples/show_resources.rs](./examples/show_resources.rs): Example code.
- [src/lib.rs](./src/lib.rs): Implementation.
- [Cargo.toml](./Cargo.toml): Metadata decribing the package and how to
  build it.

[Rust]: https://www.rust-lang.org/
