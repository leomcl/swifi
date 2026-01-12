//! Integration tests for CLI flags.

use {
    assert_cmd::Command,
    predicates::{prelude::PredicateBooleanExt, str::contains},
};

fn run_with_args(args: &[&str]) -> assert_cmd::assert::Assert {
    Command::new(env!("CARGO_BIN_EXE_swifi"))
        .args(args)
        .assert()
}

#[test]
fn test_list_flag() {
    run_with_args(&["--list"]).stdout(contains("Available Servers"));
}

#[test]
fn test_server_flag() {
    run_with_args(&["--server", "123"]).stderr(contains("Error").or(contains("not found")));
}

// ignore network tests by default (may be flaky ect)
#[test]
#[ignore]
fn test_down_flag() {
    run_with_args(&["--down"]).stdout(contains("Error").or(contains("Download Speed")));
}

#[test]
#[ignore]
fn test_up_flag() {
    run_with_args(&["--up"]).stdout(contains("Error").or(contains("Upload Speed")));
}

#[test]
#[ignore]
fn test_down_and_up_flags() {
    run_with_args(&["--down", "--up"]).stdout(
        contains("Error")
            .or(contains("Download Speed"))
            .or(contains("Upload Speed")),
    );
}

#[test]
#[ignore]
fn test_no_flags() {
    run_with_args(&[]).stdout(
        contains("Error")
            .or(contains("Download Speed"))
            .or(contains("Upload Speed")),
    );
}
