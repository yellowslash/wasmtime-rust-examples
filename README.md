# Basic examples using wasmtime with rust

There seem to be little information out there on basic examples so I try to provide them.

This project assumes a basic knowledge of the wasmtime environment, tools, concepts, etc.
It is not meant to be a kickstarter into the subject.

## Required tools

### Rust

https://www.rust-lang.org/tools/install -> $curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Wasmtime
https://wasmtime.dev -> $curl https://wasmtime.dev/install.sh -sSf | bash

### wasm32-wasip2 compilaton target
$rustup target add wasm32-wasip2

### Cargo component
$cargo install cargo-component

### WAC CLI
$cargo install wac-cli

## Notes

Some examples might not work under Windows+WSL.