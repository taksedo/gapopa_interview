use crate::value_objects::interval_duration::IntervalDuration;
use crate::value_objects::site_name::SiteName;

pub struct Request {
    pub interval_duration_millisec: IntervalDuration,
    pub site_name: SiteName,
}

impl TryFrom<Vec<String>> for Request {
    type Error = RequestError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(RequestError::ArgsQtyError);
        }

        let count_interval_argument = value.get(1).unwrap().to_string();
        let count_interval =
            IntervalDuration::try_from(count_interval_argument)?;

        let site_name_argument = value.get(2).unwrap().to_string();
        let site_name = SiteName::try_from(site_name_argument)?;

        Ok(Self {
            interval_duration_millisec: count_interval,
            site_name,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RequestError {
    SiteNameError,
    ArgsQtyError,
    IntervalValueError,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrong_interval_duration_argument() {
        let args = vec!["zero".to_string(), "one".to_string()];

        let result = Request::try_from(args);
        assert!(result.is_err());
        matches!(result, Err(RequestError::ArgsQtyError));
    }
}
