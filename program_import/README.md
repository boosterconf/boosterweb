# Program import

Imports the conference program from Sessionize into Markdown files for hugo to render.

The importer will delete all days worth of program then run an import. Therefore you can't do manual edits to the Markdown. It's a feature, not a bug. Fix the program in Sessionize if something is wrong.

## Dependencies

To run or develop program import there is only one dependency: [rustup](https://rustup.rs/), the installer for the Rust programming language which the importer is written in. Install it either from the official site or a package manager.

## Running

Run the importer by navigating into this directory with a terminal and running the following command:

`cargo run --locked`

This will download all the dependencies, compile them, then run the importer. It will take a little while the first time.

## Developing

It's just a normal Rust program. Use official tools like rust-analyzer and clippy for help. [Bacon](https://dystroy.org/bacon/) can be a nice convenience.

