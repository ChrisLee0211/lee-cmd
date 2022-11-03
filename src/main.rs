use clap::{Parser, Subcommand};

mod actions;
use crate::actions::git::{Branch, Message, git_push, git_pull, git_commit_auto_push};
use crate::actions::common::remove_cargo_cache;

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
    /// remove cargo cache
    RC
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
        Action::RC => {
            remove_cargo_cache();
        }
    }
}