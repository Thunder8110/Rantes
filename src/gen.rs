use std::{env, fs};

use anyhow::bail;

pub fn gen(problem: String) -> anyhow::Result<()> {
  let curr_dir_path = env::current_dir()?;
  let catgo_toml_path = curr_dir_path.join("Cargo.toml");
  if !catgo_toml_path.exists() {
    bail!("Cargo.toml does not exists.");
  }

  let src_bin_path = curr_dir_path.join("src").join("bin");
  match src_bin_path.try_exists() {
    Ok(true) => Ok(()),
    Ok(false) => fs::create_dir(&src_bin_path),
    Err(e) => Err(e),
  }?;

  let problem_gen_path = src_bin_path.join(format!("{}_gen.rs", &problem));
  match problem_gen_path.try_exists() {
    Ok(true) => bail!("generator for {} is exists.", &problem),
    Ok(false) => fs::File::create(&problem_gen_path),
    Err(e) => Err(e),
  }?;
  
  let solve_slow_path = src_bin_path.join(format!("{}_slow.rs", &problem));
  match solve_slow_path.try_exists() {
    Ok(true) => bail!("slow code for {} is exists.", &problem),
    Ok(false) => fs::File::create(&solve_slow_path),
    Err(e) => Err(e),
  }?;

  Ok(())
}