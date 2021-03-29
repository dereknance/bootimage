const HELP: &str = include_str!("help.txt");
const CARGO_GRUBIMAGE_HELP: &str = include_str!("cargo_grubimage_help.txt");
const RUNNER_HELP: &str = include_str!("runner_help.txt");

/// Prints a general help text.
pub fn print_help() {
    print!("{}", HELP);
}

/// Prints the help for the `cargo grubimage` command.
pub fn print_cargo_grubimage_help() {
    print!("{}", CARGO_GRUBIMAGE_HELP);
}
/// Prints the help for the `grubimage runner` command.
pub fn print_runner_help() {
    print!("{}", RUNNER_HELP);
}

/// Prints the version of this crate.
pub fn print_version() {
    println!("grubimage {}", env!("CARGO_PKG_VERSION"));
}
