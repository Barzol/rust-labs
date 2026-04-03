use std::env::{args, Args};
use std::process::exit;

pub fn run_cli (f: fn(args: Args, cmd_name: String) -> Result<(),String>) -> !{
    let mut args = args();
    let cmd = args.next().unwrap();
    let crash = |msg| {
        eprintln!("{}", msg);
        exit(1);
    };
    if let Err(msg) = f(args, cmd){crash(msg)};

    exit(0);
}