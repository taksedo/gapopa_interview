use std::fmt;
use std::fmt::Formatter;

use url::Url;

use crate::request::RequestError;

#[derive(Debug)]
pub struct SiteName {
    value: Url,
}

impl SiteName {
    pub fn get_error_code(&self) -> String {
        let error_code_raw = self.value.path();
        if error_code_raw == "/200" {
            "".to_string()
        } else if !error_code_raw.is_empty() {
            error_code_raw[1..error_code_raw.len()].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn get_host_name(&self) -> String {
        self.value.host().unwrap().to_string()
    }
}

impl TryFrom<String> for SiteName {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let site_name = Url::parse(&value);
        match site_name {
            Ok(value) => { Ok(Self { value: value }) }
            Err(_) => Err(RequestError::SiteNameError)
        }
    }
}

impl fmt::Display for SiteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::value_objects::site_name::SiteName;

    #[test]
    fn create_site_name_successful_without_response_code() {
        let site_input = "http://www.example.com".to_owned();
        let result = SiteName::try_from(site_input);
        assert!(result.is_ok());
    }

    #[test]
    fn create_site_name_successful_with_response_code_200() {
        let site_input = "http://www.example.com/200".to_owned();
        let result = SiteName::try_from(site_input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_error_code(), "".to_string());
    }
    #[test]
    fn create_site_name_successful_with_response_code() {
        let site_input = "http://www.example.com/500".to_owned();
        let result = SiteName::try_from(site_input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_error_code(), "500".to_string());
    }

    #[test]
    fn create_site_name_error() {
        let site_input = "some_random_string".to_owned();
        let result = SiteName::try_from(site_input);
        assert!(result.is_err());
    }
}