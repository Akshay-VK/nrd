use std::process::{Command,Stdio};
use config::Config;
use serde::{Serialize, Deserialize};
use dialoguer::FuzzySelect;
use std::fs::File;


#[derive(Debug, Serialize, Deserialize)]
struct Setting {
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
    let t = if cfg!(target_os = "windows"){
        ["cmd","/C"]
    }else{
        ["sh","-c"]
    };


    let config = get_config();
    let tasks = config.tasks.iter().map(|x| x.name.clone()).collect::<Vec<String>>();


    let selection = FuzzySelect::new()
        .with_prompt("Choose task:")
        .items(&tasks)
        .interact()
        .unwrap();

    let commands = &config.tasks[selection].steps;
    let to_exec = commands.join(" && ");

    let mut command = Command::new(t[0])
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

fn get_path_config()->Setting{
    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();

    let conf = settings.try_deserialize::<Setting>().expect("Unable to load path file");
    conf
}
fn get_config()->Settings{
    let config = get_path_config();

    let file = File::open(&config.path).expect("Failed to open the config file");

    let conf: Settings = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
