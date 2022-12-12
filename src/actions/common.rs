use std::{process::Command, io::{Error, ErrorKind}};
use clap::{Args};

pub fn remove_cargo_cache() {
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

#[derive(Args, Debug)]
pub struct PORT {
    #[arg(short)]
    pub port: String
}

pub fn kill_specified_port(port: &String) {
    let mut cmd_args = String::from("kill -9 ");
    cmd_args+= "$(lsof -i:";
    cmd_args+= port;
    cmd_args+= ")";
    let output = Command::new("bash").arg("-c").arg(cmd_args).output();
    match output {
        Ok(output_info) => {
            if(!output_info.status.success()) {
                let raw_output = String::from_utf8(output_info.stderr).unwrap();
                println!("kill port fail cause by {}", raw_output);
                return;
            }
            println!("kill port {} success", port);
        }
        Err(err) => {
            panic!("remove fail cause by {}", err);
        }
    }
}
