use clap::Parser;
use syncfork::run_command;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "A tool to sync the forked origin with the upstream."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(
        version = "1.0",
        author = "Mian Wu",
        about = "A tool to sync the forked origin with the upstream."
    )]
    Syncfork {
        #[clap(short, long)]
        upstream: String,
        branch: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Syncfork { upstream, branch }) => run_command(upstream, branch),
        None => println!("No command was used"),
    }
}
