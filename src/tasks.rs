use std::io::Stderr;
use std::process::Command;
use serde::{Deserialize, Serialize};
use promptuity::Promptuity;
use promptuity::prompts::Input;


#[derive(Debug,Deserialize, Serialize)]
pub struct Task{
    pub name: String,
    pub steps: Vec<String>,
    pub arguments: Option<Vec<String>>,
}


pub fn execute_command(task: &Task, p: &mut Promptuity<'_, Stderr>){
    let t = if cfg!(target_os = "windows"){
        ["cmd","/C"]
    }else{
        ["sh","-c"]
    };

    let commands = &task.steps;
    let mut to_exec = commands.join(" && ");

    let to_exec = if let Some(args) = &task.arguments {
        for arg in args {
            let entered = p.prompt(
                Input::new(format!("Enter {}: ",arg))
                    .with_default(""),
            ).expect("Error accepting argument.");

            let replacer = format!("{{{}}}",arg);
            
            to_exec = to_exec.replace(&replacer, &entered);
        }
        to_exec
    } else {
        to_exec.into()
    };

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