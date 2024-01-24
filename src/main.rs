use clap::{Parser, Subcommand};

mod gen;
mod test;

#[derive(Debug, Parser)]
struct Args {
  #[command(subcommand)]
  command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
  #[command(about = "generate random test programs")]
  Gen {
    problem: String,
  },
  #[command(about = "execute random test")]
  Test {
    problem: String,
  },
}

fn main() -> anyhow::Result<()> {
  let arg = Args::parse();
  match arg.command {
    Subcommands::Gen { problem } => gen::gen(problem),
    Subcommands::Test { problem } => test::test(problem),
  }
}
