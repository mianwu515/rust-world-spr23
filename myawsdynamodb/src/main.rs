// ! Rust CLI Tool to interact with DynamoDB using the AWS SDK for Rust
// `cargo run --bin myawsdynamodb`

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mian Wu",
    about = "AWS DynamoDB CLI in Rust",
    after_help = "Example: rust-s3-cli"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    List {
        #[clap(short, long)]
        table: Option<String>,
    },
    Create {
        #[clap(short, long)]
        table: String,
        #[clap(short, long)]
        primarykey: String,
    },
    Upload {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        filepath: String,
    },
    Delete {
        #[clap(short, long)]
        table: String,
        #[clap(short, long)]
        key: Option<String>,
    },
}

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client.list_tables().send().await?;

    println!("Tables:");

    let names = resp.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}
