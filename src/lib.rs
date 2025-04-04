// cargo_auto_template_new_cli/src/lib.rs

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_template_new_cli
//!
//! **Basic Rust project template for CLI and library, more than just -cargo new hello-**  
//! ***version: 0.0.15 date: 2025-04-04 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!  ![logo](https://raw.githubusercontent.com/automation-tasks-rs/cargo-auto/main/images/logo/logo_cargo_auto.svg)
//!  this repo is part of the [automation_tasks_rs](https://github.com/automation-tasks-rs) project
//!  
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/blob/main/LICENSE)
//!   [![Rust](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-167-green.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-88-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-60-purple.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-19-yellow.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## This template
//!
//! Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make a similar command that creates a real-life Rust project.  
//!
//! ```bash
//! cargo auto new_cli project_name
//! ```
//!
//! Extremely simple, just the basic moving parts and use cases.  
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! And code happily ever after...
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// access to modules
mod hello_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
