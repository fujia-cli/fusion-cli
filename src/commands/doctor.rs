use crate::commands::ui;
use crate::result::Result;
use bollard::Docker;
use clap::Args;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

// Executes the `ftc doctor` command to check
// the prerequisites of deployment.
#[derive(Debug, Args)]
pub struct Cmd {}

impl Cmd {
  pub async fn run(&self) -> Result {
    let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
      .unwrap()
      .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

    println!(
      "{} checking the prerequisites of deployment...",
      ui::emoji::LOOKING_GLASS
    );

    let pb = ProgressBar::new(0);
    pb.set_style(spinner_style.clone());

    for _ in 0..10 {
      pb.set_message("Checking the version of Docker...");
      pb.inc(1);
      thread::sleep(Duration::from_millis(200));
    }

    let new_spinner_style = ProgressStyle::with_template("{wide_msg}").unwrap();
    pb.set_style(new_spinner_style);
    let local_docker = Docker::connect_with_local_defaults().unwrap();

    match local_docker.version().await {
      Ok(version) => pb.finish_with_message(format!(
        "{} {}: {}\n{} {}",
        ui::emoji::SUCCESS,
        String::from("Docker version"),
        version.version.unwrap(),
        ui::emoji::SPARKLE,
        style("Success! The docker environment is ready!")
      )),
      Err(_) => pb.finish_with_message(format!(
        "{} {}\n{} {}",
        ui::emoji::FAIL,
        String::from("No running docker found."),
        ui::emoji::WARN,
        style("Please check the status of docker.").red(),
      )),
    }

    println!();
    Ok(())
  }
}
