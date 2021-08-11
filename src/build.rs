
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::Rng;
use tokio::time::sleep;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use colorful::Colorful;
use crate::shared::{COMMANDS,PACKAGES};

pub async fn subcommand() -> anyhow::Result<()> {
  let mut rng = rand::thread_rng();
  let started = Instant::now();
  let spinner_style = ProgressStyle::default_spinner()
      .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
      .template("{prefix:.bold.dim} {spinner} {wide_msg}");
  println!(
      "{} Resolving packages...",
      "[1/4]".bold().dim()
  );
  println!(
      "{} Fetching packages...",
      "[2/4]".bold().dim()
  );

  println!(
      "{} Linking dependencies...",
      "[3/4]".bold().dim()
  );

  let deps = rng.gen_range(400..1500);

  let pb = ProgressBar::new(deps);
  pb.set_style(
    ProgressStyle::default_bar()
    .progress_chars("=>-")
    .template("{spinner:.green} [{wide_bar:.green/blue}] {pos}/{len}"));

  for _ in 0..deps {
    pb.inc(1);
    sleep(Duration::from_millis(3)).await;
  }
  pb.finish_and_clear();

  println!(
    "{} Building fresh packages...",
    "[4/4]".bold().dim(),
  );

  let m = MultiProgress::new();
  let handles: Vec<_> = (0..9u32)
      .map(|i| {
          let count = rng.gen_range(200..350);
          let pb = m.add(ProgressBar::new(count));
          let styl = spinner_style.clone();
          std::thread::spawn(move || {
            new_build_pb(pb, count, i, styl)
          })
      })
      .collect();
  
  m.join()?;
  for h in handles {
      let _ = h.join();
  }
  m.join_and_clear()?;
  println!("\nDone in {}", HumanDuration(started.elapsed()));

  Ok(())
}

fn new_build_pb(pb: ProgressBar, count: u64, i: u32, spinner_style: ProgressStyle) -> anyhow::Result<()> {
  let mut rng = rand::thread_rng();
  pb.set_style(spinner_style);
  pb.set_prefix(format!("[{}/?]", i + 1));
  for _ in 0..count {
    // println!("next");
    pb.set_message(format!("{}: {}", PACKAGES.choose(&mut rng).unwrap().blue().bold(), COMMANDS.choose(&mut rng).unwrap().dark_gray()));
    pb.inc(1);
    std::thread::sleep(Duration::from_millis(rng.gen_range(150..1000)));
  }
  pb.finish_with_message("waiting...");
  Ok(())
}