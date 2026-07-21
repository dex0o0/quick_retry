use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Backoff {
    Fixed,
    Exponential(f64),
}

/// A lightweight, zero-dependency retry library for Rust.
///
/// # Example
/// ```Rust
/// use quick_retry::Retry;
/// use std::time::Duration;
///
/// let result = Retry::new()
///      .max_attempts(3)
///      .delay(Duration::from_millis(50))
///      .run(|| {
///          // your fallible oparation here
///          Ok::<_,&str>("success")
///      });
///
/// assert_eq!(result, Ok("success"));
/// ```
#[derive(Debug, Clone)]
pub struct Retry {
    max_attempts: u32,
    initial_delay: Duration,
    backoff: Backoff,
}

impl Retry {
    pub fn new() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            backoff: Backoff::Fixed,
        }
    }

    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn exponential_backoff(mut self, factor: f64) -> Self {
        self.backoff = Backoff::Exponential(factor);
        self
    }

    pub fn run<T, E, F>(&self, mut oparation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        let mut current_delay = self.initial_delay;
        let mut last_err = None;

        let success = (1..=self.max_attempts).find_map(|attempt| match oparation() {
            Ok(val) => Some(val),
            Err(e) => {
                if attempt < self.max_attempts {
                    sleep(current_delay);
                    if let Backoff::Exponential(factor) = self.backoff {
                        current_delay = current_delay.mul_f64(factor);
                    }
                }
                last_err = Some(e);
                None
            }
        });
        match success {
            Some(val) => Ok(val),
            None => Err(last_err.expect("max_attempts must be at least 1")),
        }
    }
}

impl Default for Retry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_successful_on_first_try() {
        let result = Retry::new().run(|| Ok::<_, &str>("success"));
        assert_eq!(result, Ok("success"));
    }

    #[test]
    fn test_successful_after_retries() {
        let att = AtomicI32::new(0);

        let result = Retry::new()
            .max_attempts(3)
            .delay(Duration::from_millis(10))
            .run(|| {
                let current = att.fetch_add(1, Ordering::SeqCst);
                if current < 2 {
                    Err("failed")
                } else {
                    Ok("success")
                }
            });
        assert_eq!(result, Ok("success"));
        assert_eq!(att.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_failure_after_max_attempts() {
        let att = AtomicI32::new(0);
        let result = Retry::new()
            .max_attempts(3)
            .delay(Duration::from_millis(10))
            .run(|| {
                att.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>("always fail")
            });
        assert_eq!(result, Err("always fail"));
        assert_eq!(att.load(Ordering::SeqCst), 3);
    }
}
