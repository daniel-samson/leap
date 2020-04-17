use std::env::args;

use leap::cli::{new_project, serve_project, update_cli, upgrade_project, watch_project};

/// Main entry point for the command line tool
fn main() {
    env_logger::init();
    match args().nth(1usize) {
        None => print_help(),
        Some(a) => match a.as_ref() {
            "new" => run_subcommand_new(),
            "update" => run_subcommand_update(),
            "upgrade" => run_subcommand_upgrade(),
            "serve" => run_subcommand_serve(),
            "watch" => run_subcommand_watch(),
            _ => print_help(),
        },
    }
}

/// Prints the help text
fn print_help() {
    println!(include_str!("help.txt"));
}

/// Perform the new sub-command
fn run_subcommand_new() {
    match args().nth(2usize) {
        None => print_help_subcommand_new(),
        Some(arg) => match arg.as_ref() {
            "--help" => print_help_subcommand_new(),
            "-h" => print_help_subcommand_new(),
            _ => new_project(arg),
        },
    }
}

/// Prints the help text for the new sub-command
fn print_help_subcommand_new() {
    println!(include_str!("help_subcommand_new.txt"));
}

/// Perform the update sub-command
fn run_subcommand_update() {
    match args().nth(2usize) {
        Some(arg) => match arg.as_ref() {
            "--help" => print_help_subcommand_update(),
            "-h" => print_help_subcommand_update(),
            _ => update_cli(),
        },
        _ => update_cli(),
    }
}

/// Prints the help text for the update sub-command
fn print_help_subcommand_update() {
    println!(include_str!("help_subcommand_update.txt"));
}

/// Perform the upgrade sub-command
fn run_subcommand_upgrade() {
    match args().nth(2usize) {
        Some(arg) => match arg.as_ref() {
            "--help" => print_help_subcommand_upgrade(),
            "-h" => print_help_subcommand_upgrade(),
            _ => upgrade_project(),
        },
        _ => upgrade_project(),
    }
}

/// Prints the help text for the upgrade sub-command
fn print_help_subcommand_upgrade() {
    println!(include_str!("help_subcommand_upgrade.txt"));
}

/// Perform the serve sub-command
fn run_subcommand_serve() {
    match args().nth(2usize) {
        Some(arg) => match arg.as_ref() {
            "--help" => print_help_subcommand_serve(),
            "-h" => print_help_subcommand_serve(),
            _ => serve_project(),
        },
        _ => serve_project(),
    }
}

/// Prints the help text for the serve sub-command
fn print_help_subcommand_serve() {
    println!(include_str!("help_subcommand_serve.txt"));
}

/// Perform the watch sub-command
fn run_subcommand_watch() {
    match args().nth(2usize) {
        Some(arg) => match arg.as_ref() {
            "--help" => print_help_subcommand_watch(),
            "-h" => print_help_subcommand_watch(),
            _ => watch_project(),
        },
        _ => watch_project(),
    }
}

/// Prints the help text for the watch sub-command
fn print_help_subcommand_watch() {
    println!(include_str!("help_subcommand_watch.txt"));
}
