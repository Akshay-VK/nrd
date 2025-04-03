use std::io::Stderr;
use std::process::Command;
use serde::Deserialize;
use promptuity::Promptuity;
use promptuity::prompts::{Select, SelectOption};


#[derive(Debug,Deserialize)]
pub struct Task{
    pub name: String,
    pub steps: Vec<String>,
}


pub fn execute_command(task: &Task, p: &mut Promptuity<'_, Stderr>){
    let t = if cfg!(target_os = "windows"){
        ["cmd","/C"]
    }else{
        ["sh","-c"]
    };

    let commands = &task.steps;
    let to_exec = commands.join(" && ");

    p.info(format!("Executing task: {}", task.name)).expect("Could not log");

    let command = Command::new(t[0])
        .arg(t[1])
        .arg(to_exec)
        .status()
        .expect("Failed to start interactive command");

    if command.success() {
        p.success("Interactive session completed.").expect("Could not log");
    } else {
        p.error(format!("Interactive session failed with status: {}", command)).expect("Could not log");
    }
}