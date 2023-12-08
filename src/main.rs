use actions::common::get_public_ip;
use actions::git::create_new_branch;
use clap::{Parser, Subcommand};

mod actions;
use crate::actions::git::{Branch, Message, git_push, git_pull, git_commit_auto_push, get_and_copy_current_branch};
use crate::actions::common::{remove_cargo_cache, kill_specified_port, PORT};

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
    /// git commit -m "something" and auto git push origin xxx
    CPH(Message),
    /// git checkout -b "branch name"
    CB(Branch),
    /// remove cargo cache
    RC,
    /// copy current git branch name
    BR,
    /// kill specified port
    KP(PORT),
    /// get public Ip
    IP
}


fn main() {
    let cli = Commands::parse();

    match &cli.action {
        Action::PH(Branch { branch }) => {
            git_push(&branch);
        }
        Action::PL(Branch { branch }) => {
            git_pull(&branch);
        }
        Action::CPH(Message { message }) => {
            git_commit_auto_push(message);
        }
        Action::CB(Branch {branch}) => {
            create_new_branch(branch)
        }
        Action::RC => {
            remove_cargo_cache();
        }
        Action::BR => {
            get_and_copy_current_branch()
        }
        Action::KP(PORT {port}) => {
            kill_specified_port(port);
        }
        Action::IP => {
            get_public_ip();
        }
    }
}