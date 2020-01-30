extern crate clap;
use clap::{App, Arg};
use structopt::StructOpt;
use std::process::Command;
use std::string::String;

#[derive(StructOpt)]
struct Cli {
    command1: String,
    command2: String
}

fn main() { 

    let args = Cli::from_args();

    //Get command lines
    
    let command1 = &args.command1;
    let command2 = &args.command2;

    //First command 
    let ouptut1 = Command::new(command1)
                    .output()
                    .expect("First command failed");

    let outputString = String::from_utf8_lossy(&ouptut1.stdout);
    let mut split = outputString.split(" ");

    //Second command
    Command::new(command2)
        .args(split)
        .spawn()
        .expect("Second command failed");
}

