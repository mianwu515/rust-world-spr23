use clap::Parser;
use syncfork::run_command;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "A tool to sync between upstream, forked origin and local."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Syncfork {
        #[clap(short, long)]
        path: String,
        #[clap(short, long)]
        upstream: String,
        #[clap(short, long)]
        branch: String,
    },
}

fn main() {
    println!("Usage:");
    println!("cargo run -- syncfork --path [your-local-project-path] --upstream [upstream-url] --branch [your-remote-repo-branch]");
    println!("For example:");
    println!("cargo run -- syncfork --path ~/rust-world-spr23 --upstream git@github.com:mianwu515/rust-world-spr23.git --branchmain");
    let args = Cli::parse();
    match args.command {
        Some(Commands::Syncfork { path, upstream, branch }) => run_command(path, upstream, branch),
        None => println!("No command was used"),
    }
}
