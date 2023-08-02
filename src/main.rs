#![allow(unused)]

use clap::{Parser, Subcommand};
use fusion_cli::{
  commands::{clean, doctor, reset, start},
  result::Result,
};
use std::process;

// mod commands;

#[derive(Debug, Parser)]
#[clap(name = "ftc")]
#[clap(version)]
struct Cli {
  #[clap(subcommand)]
  cmd: Cmds,
}

#[derive(Debug, Subcommand)]
enum Cmds {
  Clean(clean::Cmd),
  Doctor(doctor::Cmd),
  Reset(reset::Cmd),
  Start(start::Cmd),
}

#[tokio::main]
async fn main() {
  let cli = Cli::parse();
  if let Err(e) = run(cli).await {
    eprintln!("error: {e:?}");

    process::exit(1);
  }
}

async fn run(cli: Cli) -> Result {
  match cli.cmd {
    Cmds::Clean(cmd) => cmd.run().await,
    Cmds::Doctor(cmd) => cmd.run().await,
    Cmds::Reset(cmd) => cmd.run().await,
    Cmds::Start(cmd) => cmd.run().await,
  }
}
