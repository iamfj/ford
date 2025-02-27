use crate::libs::utils::git::{list_branches, switch_branch};
use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;

#[derive(Parser)]
#[command(about = "Switch to a branch in the stack")]
pub struct Switch {
  #[arg(help = "Name of the branch to switch to (optional)")]
  branch: Option<String>,
}

impl Switch {
  pub fn run(&self) -> Result<()> {
    let repo = Repository::open_from_env()?;

    let target_branch = match &self.branch {
      Some(branch) => branch.clone(),
      None => {
        let branches = list_branches(&repo)?;
        let selection = Select::with_theme(&ColorfulTheme::default())
          .with_prompt("Select a branch to switch to")
          .items(&branches)
          .default(0)
          .interact()?;
        branches[selection].clone()
      }
    };

    switch_branch(&repo, &target_branch)?;
    println!("Switched to branch '{}'", target_branch);
    Ok(())
  }
}
