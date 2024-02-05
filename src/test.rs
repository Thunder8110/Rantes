use std::{io::{Read, Write}, process, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};

use anyhow::bail;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal};
use indicatif::{ProgressBar, ProgressStyle};

pub fn test(problem: String, iteration: usize) -> anyhow::Result<()> {
  check_bin(&problem)?;

  let stop = Arc::new(AtomicBool::new(false));
  let stop_clone = stop.clone();
  thread::spawn(move || {
    loop {
      match event::read() {
        Ok(Event::Key(KeyEvent {
          code: KeyCode::Char('q'),
          ..
        }))
        | Ok(Event::Key(KeyEvent {
          code: KeyCode::Char('c'),
          modifiers: KeyModifiers::CONTROL,
          ..
        })) => {
          stop_clone.store(true, Ordering::SeqCst);
          break Ok(());
        },
        Ok(_) => (),
        Err(e) => bail!(e),
      }
      thread::sleep(Duration::from_millis(50));
    }
  });

  terminal::enable_raw_mode()?;

  if iteration == 0 {
    let progress_bar = ProgressBar::new(u64::MAX);
    progress_bar.set_style(
      ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {pos:>7}")
      .unwrap()
      .progress_chars("##.")
    );
    for iteration_count in 0usize.. {
      if stop.load(Ordering::SeqCst) {
        break;
      }
      
      match exec_test(&problem) {
        Ok(()) => anyhow::Ok(()),
        Err(e) => bail!("test failed in iteration {}:\n{}", iteration_count, e),
      }?;
      progress_bar.inc(1);
    }
    progress_bar.abandon();
  }else {
    let progress_bar = ProgressBar::new(iteration as u64);
    progress_bar.set_style(
      ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {pos:>7}/{len:7}")
      .unwrap()
      .progress_chars("##.")
    );
    for iteration_count in 0..iteration {
      if stop.load(Ordering::SeqCst) {
        break;
      }
      
      match exec_test(&problem) {
        Ok(()) => anyhow::Ok(()),
        Err(e) => bail!("test failed in iteration {}:\n{}", iteration_count, e),
      }?;
      progress_bar.inc(1);
    }
    progress_bar.abandon();
  }

  terminal::disable_raw_mode()?;

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