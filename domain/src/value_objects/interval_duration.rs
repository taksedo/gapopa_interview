use std::fmt;
use std::fmt::Formatter;

use crate::request::RequestError;

pub struct IntervalDuration {
    value: u64,
}

impl IntervalDuration {
    pub fn to_millis(&self) -> u64 {
        self.value
    }
}

impl TryFrom<String> for IntervalDuration {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<u8>() {
            Ok(count) => Ok(Self { value: count as u64 * 1000 }),
            Err(_) => Err(RequestError::IntervalValueError)
        }
    }
}

impl From<u64> for IntervalDuration {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl fmt::Display for IntervalDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod test {
    use rand::{Rng, thread_rng};

    use crate::value_objects::interval_duration::IntervalDuration;

    #[test]
    fn create_interval_duration_successful() {
        let milliseconds_interval = thread_rng().gen_range(0..u8::MAX);
        let milliseconds_interval_str = milliseconds_interval.to_string();

        let result = IntervalDuration::try_from(milliseconds_interval_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, milliseconds_interval as u64 * 1000)
    }

    #[test]
    fn create_interval_duration_error() {
        let milliseconds_interval_str = "some_string".to_string();

        let result = IntervalDuration::try_from(milliseconds_interval_str);
        assert!(result.is_err());
    }
}
