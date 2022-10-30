use clap::{Parser, Subcommand, Args};
use std::process::{Command};

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
#[derive(Args, Debug)]
struct Branch {
    #[arg(short, required = false)]
    branch: Option<String>,
}

fn excute_git_push(branch_str: &String) {
    let output = Command::new("git").args(["push", "origin", branch_str]).output();
    match output {
        Ok(output_info) => {
            if !output_info.status.success() {
                let raw_output = String::from_utf8(output_info.stderr).unwrap();
                println!("push fail cause by {}", raw_output);
                return;
            }
            println!("push success!!!",);
        }
        Err(err) => {
            panic!("git push fail cause by {}", err);
        }
    }
}

fn git_push(args: &Option<String>) {
    match args {
        Some(branch) => {
            excute_git_push(branch)
        }
        None => {
            let current_branch_ouput = Command::new("git").args(["symbolic-ref", "--short", "HEAD"]).output().ok().unwrap();
            let branch = String::from_utf8(current_branch_ouput.stdout).unwrap().trim().to_string();
            excute_git_push(&branch)

        }
    }
}

fn git_pull(args: &Option<String>) {

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