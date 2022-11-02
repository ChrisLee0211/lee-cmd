use std::{process::Command};
use clap::{Args};



#[derive(Args, Debug)]
pub struct Branch {
    #[arg(short, required = false)]
   pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct Message {
    #[arg(short)]
    pub message: String
}

pub fn git_push(args: &Option<String>) {
    excute_git_command(args, "push");
}

pub fn git_pull(args: &Option<String> ){
    excute_git_command(args, "pull");
}

pub fn git_commit(msg: &String) -> Result<(), String> {
    let commit_output = Command::new("git").args(["commit","-m",msg]).output().expect("fail to commit");
    if commit_output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8(commit_output.stderr).unwrap())
    }
}

pub fn git_commit_auto_push(msg: &String) {
    let commit_result = git_commit(msg);
    match commit_result {
        Ok(()) => {
            git_push(&None)
        }
        Err(err) => {
            panic!("git command run failed cause by {}", err);
        }
    }
}

pub fn format_branch(args: &Option<String>) -> String {
    match args {
        Some(branch) => branch.to_string(),
        None => {
            let current_branch_ouput = Command::new("git")
                .args(["symbolic-ref", "--short", "HEAD"])
                .output()
                .ok()
                .unwrap();
            let branch = String::from_utf8(current_branch_ouput.stdout)
                .unwrap()
                .trim()
                .to_string();
           branch
        }
    }
}

pub fn excute_git_command(args: &Option<String>, action: &str) {
    let branch = format_branch(args);
    let output = Command::new("git")
        .args([action, "origin", &branch])
        .output();
    match output {
        Ok(output_info) => {
            if !output_info.status.success() {
                let raw_output = String::from_utf8(output_info.stderr).unwrap();
                println!("git command failed cause by {}", raw_output);
                return;
            }
            println!("git command run success!!!",);
        }
        Err(err) => {
            panic!("git command run failed cause by {}", err);
        }
    }
}