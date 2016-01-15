extern crate chan;
extern crate notify;
extern crate time;

use guard;
use self::chan::Sender;
use self::notify::{RecommendedWatcher, Error, Watcher};
use self::notify::op;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc::channel;


pub struct FileOp {
  pub path: PathBuf,
  pub operator: op::Op,
}


pub fn watch(watchpath: &str, sx: Sender<FileOp>) {
  let (tx, rx) = channel();

  let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

  let mut watcher = match w {
    Ok(i) => i,
    Err(_) => {
      println!("Failed to init notify!");
      process::exit(1);
    },
  };

  let _ = watcher.watch(watchpath);

  loop {
    if let Ok(e) = rx.recv() {
      if let Ok(fop) = guard::validate(e) {
        sx.send(fop);
      }
    }
  }
}
