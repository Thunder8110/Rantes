use std::{env, io::{Read, Write}, process};

use anyhow::{anyhow, bail};

pub fn test(problem: String, iteration: usize) -> anyhow::Result<()> {
  check_bin(&problem)?;

  if iteration == 0 {
    for iteration_count in 0usize.. {
      match exec_test(&problem) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("test failed in iteration {}:\n{}", iteration_count, e))
      }?
    }
  }else {
    for iteration_count in 0..iteration {
      match exec_test(&problem) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("test failed in iteration {}:\n{}", iteration_count, e))
      }?
    }
  }

  println!("iteration finished with no failure.");
  Ok(())
}

fn check_bin(problem: &String) -> anyhow::Result<()> {
  bin_is_ok(format!("{}_gen", &problem))?;
  bin_is_ok(format!("{}_slow", &problem))?;
  bin_is_ok(format!("{}", &problem))?;

  Ok(())
}

fn bin_is_ok(name: String) -> anyhow::Result<()> {
  let check_proc = process::Command::new("cargo")
  .arg("check")
  .arg("--bin")
  .arg(&name)
  .output()?;

  if check_proc.status.success() {
    Ok(())
  }else {
    bail!("{} bin does not exist or be not compilable.", &name)
  }
}

fn exec_test(problem: &String) -> anyhow::Result<()> {
  let gen_proc = process::Command::new("cargo")
  .arg("run")
  .arg("--quiet")
  .arg("--bin")
  .arg(format!("{}_gen", &problem))
  .stdout(process::Stdio::piped())
  .output()?;

  let input = String::from_utf8_lossy(&gen_proc.stdout).to_string();

  let mut solve_proc = process::Command::new("cargo")
  .arg("run")
  .arg("--quiet")
  .arg("--bin")
  .arg(format!("{}", &problem))
  .stdin(process::Stdio::piped())
  .stdout(process::Stdio::piped())
  .spawn()?;

  solve_proc.stdin.as_mut().take().unwrap().write(input.as_bytes())?;

  let mut output_solver = String::new();
  solve_proc.stdout.take().unwrap().read_to_string(&mut output_solver)?;

  let mut slow_proc = process::Command::new("cargo")
  .arg("run")
  .arg("--quiet")
  .arg("--bin")
  .arg(format!("{}_slow", &problem))
  .stdin(process::Stdio::piped())
  .stdout(process::Stdio::piped())
  .spawn()?;

  slow_proc.stdin.as_mut().take().unwrap().write(input.as_bytes())?;

  let mut output_slow = String::new();
  slow_proc.stdout.take().unwrap().read_to_string(&mut output_slow)?;

  if output_solver == output_slow {
    Ok(())
  }else {
    let message = [
      "test is failed.".to_string(),
      "input:".to_string(),
      format!("{}", &input),
      "expected:".to_string(),
      format!("{}", &output_slow),
      "actual:".to_string(),
      format!("{}", &output_solver),
    ];
    bail!("{}", message.join("\n"))
  }
}