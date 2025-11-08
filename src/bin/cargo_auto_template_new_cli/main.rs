//! src/bin/cargo_auto_template_new_cli/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

#[macro_use]
mod utils_mod;

// Linux terminal colors
use cargo_auto_template_new_cli_lib::{GREEN, RED, RESET, YELLOW};

use utils_mod::pos;
// Bring trait for Result into scope.
use cargo_auto_template_new_cli_lib::ResultLogError;

/// Function main() returns ExitCode.
fn main() -> std::process::ExitCode {
    match main_returns_anyhow_result() {
        Err(err) => {
            eprintln!("{}", err);
            // eprintln!("Exit program with failure exit code 1");
            std::process::ExitCode::FAILURE
        }
        Ok(()) => std::process::ExitCode::SUCCESS,
    }
}

/// Function main() returns anyhow::Result.
fn main_returns_anyhow_result() -> anyhow::Result<()> {
    utils_mod::tracing_init()?;

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                print_greet_name(greet_name);
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_greet_name(greet_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => {
                        println!("{RED}Error: {err}{RESET}");
                        tracing::error!("{RED}Error: {err}{RESET}");
                    }
                }
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `cargo_auto_template_new_cli --help`{RESET}"),
    }
    Ok(())
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo_auto_template_new_cli !
    This is a simple yet complete template for a CLI program written in Rust.{RESET}

{GREEN}cargo_auto_template_new_cli --help{RESET}
{GREEN}cargo_auto_template_new_cli print world{RESET}
{GREEN}cargo_auto_template_new_cli upper world{RESET}

    {YELLOW}This command should return an error:{RESET}
{GREEN}cargo_auto_template_new_cli upper WORLD{RESET}
  
    {YELLOW}© 2025 bestia.dev  MIT License github.com/automation--tasks--rs/cargo-auto{RESET}
"#
    );
}

/// print my name
fn print_greet_name(greet_name: &str) {
    // call the function from the `lib.rs`
    println!("{}", cargo_auto_template_new_cli_lib::format_hello_phrase(greet_name));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    // use the trait ResultErrorLog method log() to append the error to the log file
    let upper = cargo_auto_template_new_cli_lib::format_upper_hello_phrase(greet_name).log(pos!())?;
    println!("{}", upper);
    // return
    Ok(())
}
