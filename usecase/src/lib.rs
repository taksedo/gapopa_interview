use std::thread;
use std::time::Duration;

use thiserror::Error;

use domain::request::{Request, RequestError};

pub fn check_site_health_use_case(args: Vec<String>) -> Result<(), CheckHealthError> {
    let request = Request::try_from(args);

    let request = request?;

    let error_code = request.site_name.get_error_code();

    let site_name = request.site_name;
    let request_interval_millis = request.interval_duration_millisec.to_millis();

    match error_code {
        x if x.is_empty() => {
            loop {
                println!("Checking '{}'. Result: OK(200)", site_name);
                thread::sleep(Duration::from_millis(request_interval_millis));
            }
        }
        _ => {
            loop {
                println!("Checking '{}'. Result: ERR({})", site_name, error_code);
                thread::sleep(Duration::from_millis(request_interval_millis));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CheckHealthError {
    #[error("URL parsing error")]
    SiteNameError,
    #[error("Number of arguments is wrong")]
    ArgsQtyError,
    #[error("Wrong interval duration argument format")]
    IntervalValueError,
}

impl From<RequestError> for CheckHealthError {
    fn from(value: RequestError) -> Self {
        match value {
            RequestError::SiteNameError => CheckHealthError::SiteNameError,
            RequestError::ArgsQtyError => CheckHealthError::ArgsQtyError,
            RequestError::IntervalValueError => CheckHealthError::IntervalValueError
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check_site_health_use_case;

    #[test]
    fn site_name_error() {
        let args = vec!["".to_string(), "1".to_string(), "some_random_string".to_string()];
        let result = check_site_health_use_case(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "URL parsing error".to_string());
    }

    #[test]
    fn number_of_args_error() {
        let args = vec!["one".to_string(), "two".to_string()];
        let result = check_site_health_use_case(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Number of arguments is wrong".to_string());
    }

    #[test]
    fn interval_duration_parsing_error() {
        let args = vec!["".to_string(), "string".to_string(), "some_random_string".to_string()];
        let result = check_site_health_use_case(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Wrong interval duration argument format".to_string());
    }
}
