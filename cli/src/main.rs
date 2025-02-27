mod commands;
mod libs;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Commands {
  Init,
  Sync,
  Switch(crate::commands::switch::Switch),
  Restack,
  Up,
  Down,
  Log,
  Submit,
}

#[derive(Parser)]
#[command(name = "ford")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::Init => todo!(),
    Commands::Sync => todo!(),
    Commands::Switch(cmd) => cmd.run(),
    Commands::Restack => todo!(),
    Commands::Up => todo!(),
    Commands::Down => todo!(),
    Commands::Log => todo!(),
    Commands::Submit => todo!(),
  }
}
