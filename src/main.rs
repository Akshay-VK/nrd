use std::process::Command;
use config::Config;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use std::fs::File;

use promptuity::prompts::{Select, SelectOption};
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

mod cli;
use cli::{CLI, BaseCommand};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    path: String
}

#[derive(Debug,Deserialize)]
struct Task{
    name: String,
    steps: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    tasks: Vec<Task>,
}

fn main() {

    let matches = CLI::from_args();
    let config = get_config();

    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    match matches.command{
        Some(command)=>{
            match command {
                BaseCommand::Run { task } => {
                    if let Some(task_name) = task {
                        if let Some(selection) = config.tasks.iter().position(|x| x.name == task_name) {
                            println!("Running task: {}", task_name);
                            execute_command(&config.tasks[selection]);
                        }else{
                            println!("Task not found: {}", task_name);
                        }
                    } else {
                        let tasks = config.tasks.iter().map(|x| x.name.clone()).collect::<Vec<String>>();

                        p.begin().expect("Failed to start prompt");
                        let selection = p.prompt(
                            Select::new(
                                "Choose task to run",
                                tasks.iter().enumerate()
                                    .map(|(i, x)| SelectOption::new(x.clone(), i))
                                    .collect::<Vec<SelectOption<usize>>>(),
                            )
                            .as_mut(),
                        ).expect("Failed to prompt for task");
                        p.finish().expect("Failed to finish prompt");

                        println!("Running task: {}", config.tasks[selection].name);
                        execute_command(&config.tasks[selection]);
                    }
                }
                BaseCommand::Serve { path } => {
                    if let Some(p) = path {
                        println!("Serving path: {}", p);
                    }else{
                        println!("No path specified.");
                    }
                }
            }
        }
        None=>{
            println!("No command provided.");
            
            // p.begin().expect("Failed to start prompt");
            // let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
            // p.finish().expect("Failed to finish prompt");

        }
    }
}

fn execute_command(task: &Task){
    let t = if cfg!(target_os = "windows"){
        ["cmd","/C"]
    }else{
        ["sh","-c"]
    };

    let commands = &task.steps;
    let to_exec = commands.join(" && ");

    let command = Command::new(t[0])
        .arg(t[1])
        .arg(to_exec)
        .status()
        .expect("Failed to start interactive command");

    if command.success() {
        println!("Interactive session completed.");
    } else {
        println!("Interactive session failed with status: {}", command);
    }
}

fn get_path_config()->AppConfig{
    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();

    let conf = settings.try_deserialize::<AppConfig>().expect("Unable to load path file");
    conf
}
fn get_config()->Settings{
    let config = get_path_config();

    let file = File::open(&config.path).expect("Failed to open the config file");

    let conf: Settings = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
