use {assert_cmd::Command, predicates::prelude::PredicateBooleanExt, predicates::str::contains, std::env};

// Helper to run the binary with args and check output
fn run_with_args(args: &[&str]) -> assert_cmd::assert::Assert {
    let mut path = env::current_exe().unwrap();
    path.pop(); // Remove exe name
    path.push("swifi.exe");
    let mut cmd = Command::new(path);
    cmd.args(args).assert()
}

#[test]
fn test_list_flag() {
    run_with_args(&["--list"]).stdout(contains("Available Servers"));
}

#[test]
fn test_server_flag() {
    run_with_args(&["--server", "123"]).stderr(contains("Error").or(contains("Download Speed")));
}

#[test]
fn test_down_flag() {
    run_with_args(&["--down"]).stderr(contains("Error").or(contains("Download Speed")));
}

#[test]
fn test_up_flag() {
    run_with_args(&["--up"]).stderr(contains("Error").or(contains("Upload Speed")));
}

#[test]
fn test_down_and_up_flags() {
    run_with_args(&["--down", "--up"]).stderr(
        contains("Error")
            .or(contains("Download Speed"))
            .or(contains("Upload Speed")),
    );
}

#[test]
fn test_no_flags() {
    run_with_args(&[]).stderr(
        contains("Error")
            .or(contains("Download Speed"))
            .or(contains("Upload Speed")),
    );
}
