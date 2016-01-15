extern crate notify;


use self::notify::Event;
use self::notify::op;
use std::fs::metadata;
use std::path::PathBuf;
use watcher::FileOp;


pub enum Error {
  IgnoredPath,
  IgnoredOperator,
  NotAFile,
  Unknown,
}

fn validate_path(path: PathBuf) -> Result<PathBuf, Error> {
  match metadata(&path) {
    Ok(m)  => if m.is_file() { Ok(path) } else { Err(Error::NotAFile) },
    Err(_) => Err(Error::Unknown),
  }
}

fn validate_operator(operator: op::Op) -> Result<op::Op, Error> {
  match operator {
    op::CREATE => Ok(op::CREATE),
    op::WRITE => Ok(op::WRITE),
    op::CHMOD | op::REMOVE | op::RENAME => Err(Error::IgnoredOperator),
    _ => Err(Error::Unknown),
  }
}

pub fn validate(e: Event) -> Result<FileOp, Error> {
  let operator = match e.op {
    Ok(o)  => validate_operator(o),
    Err(_) => Err(Error::IgnoredOperator),
  };
  let path = match e.path {
    Some(p) => validate_path(p),
    None    => Err(Error::IgnoredPath),
  };

  match operator {
    Ok(o)  => match path {
      Ok(p)  => Ok(FileOp { path: p, operator: o }),
      Err(e) => Err(e),
    },
    Err(e) => Err(e),
  }
}
