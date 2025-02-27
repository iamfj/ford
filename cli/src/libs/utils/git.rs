use anyhow::{bail, Result};
use git2::{BranchType, Repository};

pub fn switch_branch(repo: &Repository, branch_name: &str) -> Result<()> {
  let branch_ref = format!("refs/heads/{}", branch_name);
  repo.revparse_single(&branch_ref)?;
  repo.set_head(&branch_ref)?;
  repo.checkout_head(None)?;

  Ok(())
}

pub fn list_branches(repo: &Repository) -> Result<Vec<String>> {
  let branches: Vec<String> = repo
    .branches(Some(BranchType::Local))?
    .filter_map(|b| {
      if let Ok((branch, _)) = b {
        branch.name().ok().flatten().map(String::from)
      } else {
        None
      }
    })
    .collect();
  if branches.is_empty() {
    bail!("No branches found in the repository");
  }
  Ok(branches)
}
