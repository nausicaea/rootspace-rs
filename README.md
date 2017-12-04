# Rootspace

I originally tried to write the game in a mix of Python and C, but moved to
Rust because of its wonderful type system and the resulting guarantees.

# Prerequisites

You must have a recent version of nightly rust to build the project. Ideally,
go to [rustup.rs](https://www.rustup.rs/) and follow the instructions to obtain
rust and cargo. Currently, the project builds and runs on all architectures
supported by rust itself.

Then, you have to install nightly rust with

    $ rustup toolchain install nightly

And, in most cases, you want to set nightly rust to the default toolchain with

    $ rustup default nightly

# Build

Clone the repository with

    $ git clone https://github.com/nausicaea/rootspace-rs.git

Then, build the project with

    $ cd rootspace-rs
    $ cargo build [--release]

# Run

Run the project in debug mode with

    $ cargo run -- -vvvv -d

or get help with

    $ caro run -- --help
