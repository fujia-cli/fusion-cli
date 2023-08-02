use crate::result::Result;
use clap::Args;

pub struct Cmd {}

impl Cmd {
  pub async fn run(&self) -> Result {
    println!("start command");

    Ok(())
  }
}
