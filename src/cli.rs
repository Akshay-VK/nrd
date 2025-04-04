use serde::{Serialize,Deserialize};
use structopt::StructOpt;
use config::Config;
use std::fs::File;
use std::io::Stderr;
use promptuity::Promptuity;
use promptuity::prompts::{Select, SelectOption};

use dirs::config_dir;
use std::path::PathBuf;

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
            // p.begin().expect("Failed to start prompt");
            let selection = p.prompt(
                Select::new(
                    "Choose task to run",
                    tasks.iter().enumerate()
                        .map(|(i, x)| SelectOption::new(x.clone(), i))
                        .collect::<Vec<SelectOption<usize>>>(),
                )
                .as_mut(),
            ).expect("Failed to prompt for task");
            // p.finish().expect("Failed to finish prompt");
    
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
            pr.info(format!("Updating config path: {}", p)).expect("Could not log");
            
        }else{
            pr.error("No path specified.").expect("Could not log");
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
    let base_dir = config_dir();
    let base_dir = match base_dir {
        Some(path) => path.join("nrd").join("config.yaml"),
        None => panic!("Unable to load base directory"),
    };

    if !base_dir.exists() {
        // panic!("Config file does not exist at: {:?}", base_dir);
        std::fs::create_dir_all(base_dir.parent().unwrap()).expect("Failed to create config nrd directory");
        std::fs::write(&base_dir, "path: ./settings.yaml").expect("Failed to create config file");
        let default_config = Settings {
            tasks: vec![],
        };
        std::fs::write(base_dir.parent().unwrap().join("settings.yaml"), serde_yml::to_string(&default_config).unwrap()).expect("Failed to create config file");

    }

    // let settings = Config::builder()
    //     .add_source(config::File::with_name("settings"))
    //     .build()
    //     .unwrap();

    // let conf = settings.try_deserialize::<AppConfig>().expect("Unable to load path file");
    // conf
    let file = File::open(&base_dir).expect("Failed to open the config file");
    let conf: AppConfig = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
pub fn get_config()->Settings{
    let config = get_path_config();

    let file = File::open(&config.path).expect(format!("Failed to open the config file: {}", config.path).as_str());

    let conf: Settings = serde_yml::from_reader(file).expect("Error occured while parsing settings.");
    conf
}
