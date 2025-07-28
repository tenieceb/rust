# Overview


This project is a Rust-based emotion tracker that allows users to log daily emotional entries, each tagged with a date, emotion, color (based on emotion), and an optional note. The entries are saved into a plain text file for future reference.

The purpose of this software was to demonstrate core Rust concepts such as variables, conditionals, loops, functions with ownership, and object-oriented features using structs and impl blocks. It helped reinforce my understanding of Rust's strict type system, ownership rules, and modular structure.

[Software Demo Video](https://www.youtube.com/watch?v=6_6oHhLony8)

# Development Environment

Code Editor: Visual Studio Code

Compiler: rustc (Rust compiler)

Toolchain: Rustup with stable channel

Terminal: PowerShell & Cargo for running and testing

This project was developed in the Rust programming language. No external libraries were used — only the Rust standard library and modules written for the application (e.g., storage.rs, display.rs).

# Useful Websites

- [The Rust Book (official)](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/)

# Future Work

- Store data in a local SQLite or cloud database instead of a flat text file
- Create a web interface using Rust + WebAssembly or integrate with a web frontend
- Encrypt data for privacy and security
- Create a device app that will link to the website data and have more options for the user to set up daily reminders, save fabric name/yarn name, stitch or pattern, etc.
- Catch potential input errors
- Upgrade it to also include the option to create a blanket pattern using the user's choice of weather data. Also known as a temperature blanket.