use anyhow::{bail, Context, Result};
use config::{Config, FileFormat};
use git2::Repository;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Configuration file name
const CONFIG_FILE_NAME: &str = ".stackrc";

/// Supported configuration file formats and their extensions
const CONFIG_EXTENSIONS: &[(FileFormat, &str)] = &[
  (FileFormat::Toml, ".toml"),
  (FileFormat::Json, ".json"),
  (FileFormat::Json, ".jsonc"),
  (FileFormat::Yaml, ".yaml"),
];

/// Default configuration file format
const DEFAULT_FORMAT: FileFormat = FileFormat::Toml;

/// Configuration for the stack
#[derive(Debug, Deserialize)]
pub struct StackConfig {
  /// The repository URL or identifier
  pub repository: String,
}

/// Handles discovery and loading of configuration files
#[derive(Debug)]
pub struct ConfigLoader {
  root_path: PathBuf,
}

impl ConfigLoader {
  /// Creates a new ConfigLoader from the current directory
  pub fn new() -> Result<Self> {
    let repo = Repository::discover(".")
      .context("Not a git repository. Please initialize git first.")?;

    let root_path = repo
      .workdir()
      .ok_or_else(|| anyhow::anyhow!("Could not find git root directory"))?
      .to_path_buf();

    Ok(Self { root_path })
  }

  /// Finds the configuration file in the git root directory
  fn find_config_file(&self) -> Result<(PathBuf, FileFormat)> {
    for (format, ext) in CONFIG_EXTENSIONS {
      let rc_path = self.root_path.join(format!("{}{}", CONFIG_FILE_NAME, ext));
      if rc_path.exists() {
        return Ok((rc_path, *format));
      }
    }

    // Try default format without extension
    let default_path = self.root_path.join(CONFIG_FILE_NAME);
    if default_path.exists() {
      return Ok((default_path, DEFAULT_FORMAT));
    }

    bail!(
      "Stack config file ({}) not found in repository root",
      CONFIG_FILE_NAME
    );
  }

  /// Loads and validates the configuration
  pub fn load_config(&self) -> Result<StackConfig> {
    let (rc_path, format) = self.find_config_file()?;

    let config = Config::builder()
      .add_source(config::File::from(rc_path.as_path()).format(format))
      .build()
      .context("Failed to build configuration")?;

    let config = config
      .try_deserialize::<StackConfig>()
      .context("Failed to deserialize configuration")?;

    self.validate_config(&config, &rc_path)?;

    Ok(config)
  }

  /// Validates the configuration
  fn validate_config(
    &self,
    config: &StackConfig,
    rc_path: &Path,
  ) -> Result<()> {
    if config.repository.is_empty() {
      bail!(
        "Configuration file '{}' is missing required 'repository' field",
        rc_path.display()
      );
    }
    Ok(())
  }
}

impl StackConfig {
  /// Loads the stack configuration from the current directory
  pub fn load() -> Result<Self> {
    let loader = ConfigLoader::new()?;
    loader.load_config()
  }
}
