use structopt::StructOpt;

mod cli;
mod client;
mod server;

use cli::{CommandLineArgs, Mode::*};

fn main() {
    // println!("{:#?}", cli::CommandLineArgs::from_args());
    let CommandLineArgs { mode } = CommandLineArgs::from_args();

    match mode {
        Server => server::start(),
        Client => client::start(),
        Dual => panic!("Dual mode not implemented"),
    };
}
