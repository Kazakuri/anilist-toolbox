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
    let nanos = self.0.subsec_nanos();

    if secs == 0 && nanos == 0 {
      f.write_str("0s")?;
      return Ok(());
    }

    let ydays = secs % 31_557_600;
    let mdays = ydays % 2_630_016;
    let days = mdays / 86400;
    let day_secs = mdays % 86400;
    let hours = day_secs / 3600;
    let minutes = day_secs % 3600 / 60;
    let seconds = day_secs % 60;

    let millis = nanos / 1_000_000;
    let micros = nanos / 1000 % 1000;
    let nanosec = nanos % 1000;

    let ref mut started = false;

    item(f, started, "d", days as u32)?;
    item(f, started, "h", hours as u32)?;
    item(f, started, "m", minutes as u32)?;
    item(f, started, "s", seconds as u32)?;
    item(f, started, "ms", millis)?;
    item(f, started, "us", micros)?;
    item(f, started, "ns", nanosec)?;

    Ok(())
  }
}