extern crate clap;
use clap::{
    Arg,
    App};
use std::process::{
    Command,
    Stdio};

fn main() {
    let args: Vec<Arg> = vec![

        Arg::with_name("in")
            .short("i")
            .long("in")
            .value_name("Input ")
            .required(true)
            .takes_value(true),
        Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("Output ")
            .required(true)
            .takes_value(true)
    ];

    let app: App = App::new("pipe")
        .version("1.0")
        .author("ourdia oualiken ")
        .args(&args);

    let matches = app
        .get_matches_safe()
        .unwrap_or_else(|e| e.exit());
    let in_command = matches
        .value_of("in")
        .unwrap();
    let out_command = matches
        .value_of("out")
        .unwrap();
    let in_process = Command::new(in_command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn IN process.");

    let mut out_process = Command::new(out_command)
        .stdin(in_process.stdout.unwrap())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed  output.");
    out_process.wait().expect("Failed to get process's status.");
}
