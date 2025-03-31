use structopt::StructOpt;

// arg_enum! {
// #[derive(Debug)]
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
}
// }

#[derive(StructOpt, Debug)]
#[structopt(name = "nrd")]
pub struct CLI{
    #[structopt(subcommand)]
    pub command: Option<BaseCommand>,
}