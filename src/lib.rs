use std::{fs::File, io::Write, sync::Mutex};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
pub use profiler_macro::*;

pub struct ProfileLogger {
  file: Mutex<File>,
}
impl ProfileLogger {
  pub fn entry(
    &self,
    inner_name: &str,
    time: std::time::Duration,
    start: std::time::SystemTime,
    end: std::time::SystemTime,
    module: &str,
  ) {
    let mut file = self.file.lock().unwrap();
    let start = DateTime::<Local>::from(start);
    let end = DateTime::<Local>::from(end);
    _ = writeln!(
      file,
      "{module}::{inner_name} => {} {} {}",
      time.as_micros(),
      start,
      end
    );
    _ = file.flush();
  }
}

lazy_static! {
  pub static ref GLOBAL_PROFILER: ProfileLogger = ProfileLogger {
    file: Mutex::new(File::create("profiler.log").unwrap())
  };
}
