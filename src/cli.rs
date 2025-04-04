use serde::{Serialize,Deserialize};
use structopt::StructOpt;
use std::fs::File;
use std::io::Stderr;
use std::path::Path;
use promptuity::Promptuity;
use promptuity::prompts::{Input, Select, SelectOption};

use dirs::config_dir;

use crate::tasks::{Task, execute_command};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    path: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub tasks: Vec<Task>,
}

#[derive(StructOpt, Debug)]
pub enum BaseCommand {
    #[structopt(alias = "run")]
    Run {
        #[structopt(short, long, help = "Name of task to run.")]
        task: Option<String>,
    },
    #[structopt(alias = "serve")]
    Serve {
        #[structopt(short, long, help = "Path to serve.")]
        path: Option<String>,
    },
    #[structopt(alias = "update_config")]
    UpdateConfig {
        #[structopt(short, long, help = "Path to new config.")]
        path: Option<String>,
    }
}

impl BaseCommand {
    pub fn run(task: Option<String>, config: &Settings, p: &mut Promptuity<'_, Stderr>){
        if let Some(task_name) = task {
            if let Some(selection) = config.tasks.iter().position(|x| x.name == task_name) {
                execute_command(&config.tasks[selection],p);
            }else{
                p.error(format!("Task not found: {}", task_name)).expect("Could not log");
            }
        } else {    // No task name provided
            let tasks = config.tasks.iter().map(|x| x.name.clone()).collect::<Vec<String>>();
    
            // Prompt user to select a task
            let selection = p.prompt(
                Select::new(
                    "Choose task to run",
                    tasks.iter().enumerate()
                        .map(|(i, x)| SelectOption::new(x.clone(), i))
                        .collect::<Vec<SelectOption<usize>>>(),
                )
                .as_mut(),
            ).expect("Failed to prompt for task");
    
            execute_command(&config.tasks[selection],p);
        }
    }
    pub fn serve(path: Option<String>, pr: &mut Promptuity<'_, Stderr>){
        if let Some(p) = path {
            pr.info(format!("Serving path: {}", p)).expect("Could not log");
            
        }else{
            pr.error("No path specified.").expect("Could not log");
        }
    }
    pub fn update_config(path: Option<String>, pr: &mut Promptuity<'_, Stderr>){
        if let Some(p) = path {

            if !Path::new(&p).exists() {
                pr.error(format!("Path does not exist: {}", p)).expect("Could not log");
                return;
            }

            pr.info(format!("Updating config path: {}", p)).expect("Could not log");

            let base_dir_opt = config_dir();
            let base_dir = match base_dir_opt{
                Some(path) => path.join("nrd").join("config.yaml"),
                None => panic!("Unable to load base directory"),
            };

            std::fs::write(&base_dir, format!("path: {}",Path::new(p.as_str()).display())).expect("Failed to create config file");
            
        }else{
            pr.info("No path specified.").expect("Could not log");

            let path = pr.prompt(
                Input::new("Enter path to new config file: ")
                    .with_hint("Enter entire path with /config.yaml")
                    .with_validator(|v:&String| {
                        if Path::new(v).exists() {
                            Ok(())
                        } else {
                            Err("Path does not exist".to_string())
                        }
                    }),
            );
            let path = match path {
                Ok(p) => p,
                Err(e) => {
                    pr.error(format!("Error: {}", e)).expect("Could not log");
                    return;
                }
            };

            pr.info(format!("Updating config path: {}", path)).expect("Could not log");

            let base_dir_opt = config_dir();
            let base_dir = match base_dir_opt{
                Some(path) => path.join("nrd").join("config.yaml"),
                None => panic!("Unable to load base directory"),
            };

            std::fs::write(&base_dir, format!("path: {}",Path::new(path.as_str()).display())).expect("Failed to create config file");
        }

    }  
}

#[derive(StructOpt, Debug)]
#[structopt(name = "nrd")]
pub struct CLI{
    #[structopt(subcommand)]
    pub command: Option<BaseCommand>,
}


pub fn get_path_config()->AppConfig{
    let base_dir_opt = config_dir();
    let base_dir = match base_dir_opt{
        Some(path) => path.join("nrd").join("config.yaml"),
        None => panic!("Unable to load base directory"),
    };

    if !base_dir.exists() {
        std::fs::create_dir_all(base_dir.parent().unwrap()).expect("failed to create config nrd directory");
        std::fs::write(&base_dir, format!("path: {}",base_dir.parent().unwrap().join("settings.yaml").display())).expect("Failed to create config file");
        let default_config = Settings {
            tasks: vec![
                Task {
                    name: "test".to_string(),
                    steps: vec!["echo Hello, World!".to_string()],
                    arguments: None,
                },
            ],
        };
        std::fs::write(base_dir.parent().unwrap().join("settings.yaml"), serde_yml::to_string(&default_config).unwrap()).expect("Failed to create config file");

    }

    let file = File::open(&base_dir).expect("Failed to open the config file");
    let conf: AppConfig = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
pub fn get_config()->Settings{
    let config = get_path_config();

    let file = File::open(&config.path).expect(format!("Failed to open the config file: {}", config.path).as_str());

    println!("Config file path: {}", config.path);
    let conf: Settings = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
