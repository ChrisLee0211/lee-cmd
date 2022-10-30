use clap::{Parser, Subcommand};
use std::process::{Command};

mod actions;
use crate::actions::git::{Branch, git_push, git_pull};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Commands {
    #[command(subcommand)]
    action: Action
}

#[derive(Subcommand, Debug)]
enum Action {
    /// git pull origin xxx
    PL(Branch),
    /// git push origin xxx
    PH(Branch),
    /// remove cargo cache
    RC
}
fn remove_cargo_cache() {
    let output = Command::new("rm").args(["-rf", "~/.cargo/.package-cache"]).output();
    match output {
        Ok(output_info) => {
            if(!output_info.status.success()) {
                let raw_output = String::from_utf8(output_info.stderr).unwrap();
                println!("remove fail cause by {}", raw_output);
                return;
            }
            println!("remove cargo package cache success")
        }
        Err(err) => {
            panic!("remove fail cause by {}", err);
        }
    }
}

fn main() {
    let cli = Commands::parse();

    match &cli.action {
        Action::PH(Branch { branch }) => {
            git_push(&branch)
        }
        Action::PL(Branch { branch }) => {

        }
        Action::RC => {
            remove_cargo_cache()
        }
    }
}