// a tool to convert input json into formatted json

// CMD-input specify a file path or a json string
// CMD-output prints formatted json data to cmd (and a file)

use clap::Parser;
use jsonformatter::read_from_string;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "A Rust CLI tool - JSON formatter"
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
        about = "A Rust CLI tool - JSON formatter"
    )]
    Jsonformatter {
        #[clap(short, long)]
        obj: String,
        output: String,
    },
}
fn main() {
    // read_from_string("{\"cmd\":3}".to_string(), "abc.txt".to_string());
    let args = Cli::parse();
    match args.command {
        Some(Commands::Jsonformatter { obj, output }) => read_from_string(obj, output),
        None => println!("No command was used"),
    }
}
