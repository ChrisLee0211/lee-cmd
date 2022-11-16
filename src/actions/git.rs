use std::{process::Command};
use clap::{Args};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

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

pub fn git_add() -> Result<(), String>{
    let commit_output = Command::new("git").args(["add","."]).output().expect("fail to add");
    if commit_output.status.success() {
        println!("git add successs!!");
        Ok(())
    }else {
        Err(String::from_utf8(commit_output.stderr).unwrap())
    }
}

pub fn git_push(args: &Option<String>) -> Result<(), String> {
   let excute_reuslt = excute_git_command(args, "push");
    excute_reuslt
}

pub fn git_pull(args: &Option<String> ){
    excute_git_command(args, "pull").expect("fail to run git pull");
}

pub fn git_commit(msg: &String) -> Result<(), String> {
    let commit_output = Command::new("git").args(["commit","-m",msg]).output().expect("fail to commit");
    if commit_output.status.success() {
        println!("git commit successs!!");
        Ok(())
    } else {
        Err(String::from_utf8(commit_output.stderr).unwrap())
    }
}

pub fn cancel_commit() -> Result<(), String> {
    let cancel_result = Command::new("git")
        .args(["reset", "--soft", "HEAD^"])
        .output().expect("reset last git commit failed");
        if cancel_result.status.success() {
            println!("reset last commit success!! please try again~");
            Ok(())
        } else {
            Err(String::from_utf8(cancel_result.stderr).unwrap())
        }
}

pub fn git_commit_auto_push(msg: &String) {
    git_add().expect("fail to run git add!");
    let commit_result = git_commit(msg);
    match commit_result {
        Ok(()) => {
            let git_push_result = git_push(&None);
            if git_push_result.is_err() {
                cancel_commit().expect("git reset fail~");
            }
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

pub fn excute_git_command(args: &Option<String>, action: &str) -> Result<(), String> {
    let branch = format_branch(args);
    let output = Command::new("git")
        .args([&action, "origin", &branch])
        .output();
    match output {
        Ok(output_info) => {
            if !output_info.status.success() {
                let raw_output = String::from_utf8(output_info.stderr).unwrap();
                println!("git command failed cause by {}", raw_output);
                Err(raw_output)
            } else {
                println!("git command run success!!!",);
                Ok(())
            }
        }
        Err(err) => {
            let msg = err.to_string();
            println!("git command run failed cause by {}", &msg);
            Err(msg)
        }
    }
}

pub fn get_and_copy_current_branch() {
    let branch_name = format_branch(&None);

    
}