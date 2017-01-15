# `resource_monitor`: Check resources available to the current process

**PROOF OF CONCEPT, design & code review highly welcome.**

The goal of this crate is to allow your Rust applications to determine
when they're _about_ to run out of memory, and start refusing requests
before they actually crash:

```rust
if Resource::Memory.available()? < MINIMUM_REQUEST_MEMORY {
    debug!("Dangerously low on memory, refusing request");
    // Return an HTTP 503 error and allow the load balancer to
    // either find another worker or shed load.
}
```

See [Queues Don't Fix Overload][backpressure] for a discussion of
"backpressure", and why you might want to start refusing connections before
things break. Also note that the available memory is an imprecise number, so
you'll want to experiment to find out how much buffer you actually need.

I believe that testing available memory before handling a request is usually
superior to using a `malloc`-style API that returns `NULL` on allocation
failure, because it's better to abort the computation at the top-level in a
single, well-tested spot than to try to recover from an allocation failure
deep in your code. Of course, if you're multi-threaded, watch out for race
conditions.

Right now, we only support checking the available RAM on a Linux system
using `cgroups`, and we require your Rust code to use `jemalloc`. This
should work inside a Docker container, or outside of a container on at least
Ubuntu 16.04.

```rust
let res = resource_monitor::Resource::Memory;
println!("Memory:");
println!("  limit: {}", res.limit().unwrap());
println!("  used: {}", res.used().unwrap());
println!("  available: {}", res.available().unwrap());
```

Note that we actually poke around in jemalloc internal stats to figure out
how much free memory is available on the jemalloc heap.

Patches to add new resource types and new kinds of limits (`getrlimit`,
etc.) are very much welcome! In particular, if submitting a PR, please
be careful to explain how the different kinds of OS limits interact, and
which limit a given process will hit first. If you make me look this up,
I may procrastinate on merging, sadly. :-)

[backpressure]: http://ferd.ca/queues-don-t-fix-overload.html

## To test what happens when memory is exhausted

Install Docker and run:

```sh
./test.sh
```

This should print something like:

```
Available: 103686144
Available: 103763968
Available: 101666816
Available: 99569664
...
Available: 13586432
Available: 11489280
Available: 11489280
Available: 9392128
Clearing
Available: 104124416
```

See [examples/use_all_memory.rs](./examples/use_all_memory.rs) for the code
we run.  Do not run this outside of a memory-limited container! It may crash
your system or cause it to swap too heavily.

## What about overcommit?

[Overcommit][] is when the Linux hands out virtual memory with no plan
for providing real memory to back it up. If it turns out the kernel guessed
wrong, it will just kill your application.  Or maybe another application on
the same machine.

Some experiments suggest that large, overcommitted blocks do not show up
when we query `cgroups` for how much RAM is in use. The only way to avoid
this is currently to either (1) initialize large memory blocks as you allocate
them, or (2) disable overcommit. (Do not disable overcommit on your dev
workstation unless you want to watch half your desktop crash immediately.)

[Overcommit]: https://www.kernel.org/doc/Documentation/vm/overcommit-accounting

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
- [src/allocator_stats.rs](./src/allocator_stats.rs): Low-level `jemalloc`
  interface using the C FFI from Rust.
- [Cargo.toml](./Cargo.toml): Metadata decribing the package and how to
  build it.

[Rust]: https://www.rust-lang.org/
