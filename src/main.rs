use clap::{Parser, Subcommand};

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

fn gen(problem: String) {
  //
}

fn test(problem: String) {
  //
}

fn main() {
  let arg = Args::parse();
  match arg.command {
    Subcommands::Gen { problem } => {
      gen(problem);
    },
    Subcommands::Test { problem } => {
      test(problem);
    },
  }
}
