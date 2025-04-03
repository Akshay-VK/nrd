use std::vec;

use structopt::StructOpt;

use promptuity::prompts::{Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term};

mod tasks;

mod cli;
use cli::{CLI, BaseCommand, get_config};


fn main() {

    let matches = CLI::from_args();
    let config = get_config();

    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.with_intro("NRD").begin().expect("Failed to start prompt");


    match matches.command{
        Some(command)=>{    // Command line argument provided
            match command {
                BaseCommand::Run { task } => BaseCommand::run(task, &config, &mut p),  // Run command chosen
                BaseCommand::Serve { path } => BaseCommand::serve(path, &mut p),    // Serve command chosen
            }
        }
        None=>{    // No command line argument provided
            p.info("No command provided.").expect("Could not log");
            // println!("No command provided.");

            let commands = vec!["Run", "Serve"];
            
            // p.begin().expect("Failed to start prompt");
            let selection = p.prompt(
                Select::new(
                    "Choose command",
                    commands.iter().enumerate()
                        .map(|(i, x)| SelectOption::new(x, i))
                        .collect::<Vec<SelectOption<usize>>>(),
                )
                .as_mut(),
            ).expect("Failed to prompt for command");
            // p.finish().expect("Failed to finish prompt");

            match selection {
                0 => BaseCommand::run(None, &config, &mut p),  // Run command chosen
                1 => BaseCommand::serve(None, &mut p),    // Serve command chosen
                _ => {p.error("Invalid command selected.").expect("Could not log");},    // Invalid command
            }
        }
        
    }
    p.finish().expect("Failed to finish prompt");
}