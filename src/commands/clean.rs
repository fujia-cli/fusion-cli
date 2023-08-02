use crate::result::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(&self) -> Result {
      println!("clean command");

      Ok(())
    }
}