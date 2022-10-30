use std::process::Command;

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