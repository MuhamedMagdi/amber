#![cfg(test)]
extern crate test_generator;
use super::test_amber;
use crate::tests::compile_code;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;

fn http_server() {
    use tiny_http::{Response, Server};

    let server = Server::http("127.0.0.1:8081").expect("Can't bind to 127.0.0.1:8081");
    if let Some(req) = server.incoming_requests().next() {
        req.respond(Response::from_string("ok"))
            .expect("Can't respond");
    }
}

#[test]
fn exit() {
    let code = fs::read_to_string("src/tests/stdlib/no_output/exit.ab")
        .expect("Failed to open stdlib/no_output/exit.ab test file");

    let code = compile_code(code);
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(code)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Couldn't spawn bash");

    assert_eq!(
        cmd.wait()
            .expect("Couldn't wait for bash to execute")
            .code(),
        Some(37)
    );
}

#[test]
fn download() {
    let server = std::thread::spawn(http_server);

    let code = fs::read_to_string("src/tests/stdlib/no_output/download.ab")
        .expect("Failed to open stdlib/no_output/download.ab test file");

    test_amber(code, "ok");

    std::thread::sleep(Duration::from_millis(150));
    assert!(server.is_finished(), "Server has not stopped!");
}
