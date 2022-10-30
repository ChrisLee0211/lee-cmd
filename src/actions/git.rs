use std::process::Command;
use clap::Args;



#[derive(Args, Debug)]
pub struct Branch {
    #[arg(short, required = false)]
   pub branch: Option<String>,
}

pub fn git_push(args: &Option<String>) {
    match args {
        Some(branch) => excute_git_push(branch),
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
            excute_git_push(&branch)
        }
    }
}

pub fn excute_git_push(branch_str: &String) {
    let output = Command::new("git")
        .args(["push", "origin", branch_str])
        .output();
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