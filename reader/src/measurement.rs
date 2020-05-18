pub struct Measurement {
    pub temperature: f32,
    pub humidity: f32,
}

use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    ConvertError(Utf8Error),
    InvalidData(String),
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Error {
        Error::ConvertError(error)
    }
}

impl std::convert::TryFrom<&[u8]> for Measurement {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Measurement, Self::Error> {
        let string = std::str::from_utf8(data)?;

        let mut parts = string.trim_end_matches("\u{0}").split(" ");
        let temperature = Self::parse_value("T", &mut parts, &string)?;
        let humidity = Self::parse_value("H", &mut parts, &string)?;

        Ok(Measurement {
            temperature,
            humidity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_parse() {
        let test_data = "T=23.2 H=34.3\u{0}";
        let measurement = Measurement::try_from(test_data.as_bytes()).expect("valid data");

        assert_eq!(measurement.temperature, 23.2);
        assert_eq!(measurement.humidity, 34.3);
    }
}

use std::str::FromStr;
impl Measurement {
    fn parse_value<'a>(
        expected_key: &'static str,
        parts: &mut impl Iterator<Item = &'a str>,
        string: &str,
    ) -> Result<f32, Error> {
        let mut parts = parts
            .next()
            .ok_or(Error::InvalidData(string.to_string()))?
            .split("=");

        let key = parts.next().ok_or(Error::InvalidData(string.to_string()))?;

        if key != expected_key {
            return Err(Error::InvalidData(string.to_string()));
        }

        let value = parts.next().ok_or(Error::InvalidData(string.to_string()))?;

        f32::from_str(value).map_err(|_| Error::InvalidData(string.to_string()))
    }
}

use std::fmt;
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T={:.1} H={:.1}", self.temperature, self.humidity)
    }
}
