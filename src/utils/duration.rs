use std::time::Duration as _Duration;
use std::fmt::{Formatter, Display, Result};

pub struct Duration(_Duration);

impl From<_Duration> for Duration {
  fn from(d: _Duration) -> Self {
    Self(d)
  }
}

fn item(f: &mut Formatter, started: &mut bool, name: &str, value: u32) -> Result {
  if value > 0 {
    if *started {
      f.write_str(" ")?;
    }

    write!(f, "{}{}", value, name)?;
    *started = true;
  }

  Ok(())
}

impl Display for Duration {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let secs = self.0.as_secs();

    if secs == 0 {
      f.write_str("0s")?;
      return Ok(());
    }

    let ydays = secs % 31_557_600;
    let days = ydays / 86400;
    let day_secs = ydays % 86400;
    let hours = day_secs / 3600;
    let minutes = day_secs % 3600 / 60;

    let ref mut started = false;

    item(f, started, "d", days as u32)?;
    item(f, started, "h", hours as u32)?;
    item(f, started, "m", minutes as u32)?;

    Ok(())
  }
}