use clap::Parser;
use rockpaperscissors::play;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "A Rock-Paper-Scissors game."
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
        about = "A Rock-Paper-Scissors game."
    )]
    Play {
        #[clap(short, long)]
        choice: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { choice }) => play(choice),
        None => println!("No command was used"),
    }
}
