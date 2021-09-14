use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Mode {
    Client,
    Server,
    Dual,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Project Backup",
    about = "Tool to help automate the task of creating non-local backups."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub mode: Mode,
}
