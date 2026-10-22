use std::time::Duration;
use thiserror::Error;

/// Supported time units for parsing durations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
}

/// Errors that can occur when parsing a duration string.
#[derive(Debug, Error)]
pub enum DurationError {
    #[error("Invalid duration: {0:?}")]
    InvalidFormat(String),

    #[error("Negative duration: {0:?}")]
    NegativeValue(String),

    #[error("Duration out of range: {0:?}")]
    OutOfRange(String),
}

/// Parses a duration string into a [`Duration`].
///
/// Supports numeric values with optional unit suffixes (`s`, `m`, `h`)
/// and `H:MM` notation. Uses `default_unit` when no suffix is provided.
///
/// # Errors
///
/// Returns [`DurationError::InvalidFormat`] for malformed inputs,
/// [`DurationError::NegativeValue`] for negative inputs, and
/// [`DurationError::OutOfRange`] for values that exceed supported limits.
pub fn parse_duration(raw: &str, default_unit: Unit) -> Result<Duration, DurationError> {
    let input = raw.trim();

    if input.is_empty() {
        return Err(DurationError::InvalidFormat(raw.to_string()));
    }
    if input.starts_with('-') {
        return Err(DurationError::NegativeValue(raw.to_string()));
    }
    if let Some((hours, minutes)) = input.split_once(':') {
        if [hours, minutes]
            .iter()
            .any(|part| part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()))
        {
            return Err(DurationError::InvalidFormat(raw.to_string()));
        }
        let hours: u64 = hours
            .parse()
            .map_err(|_| DurationError::InvalidFormat(raw.to_string()))?;
        let minutes: u64 = minutes
            .parse()
            .map_err(|_| DurationError::InvalidFormat(raw.to_string()))?;
        if minutes >= 60 {
            return Err(DurationError::OutOfRange(raw.to_string()));
        }

        let seconds = hours
            .checked_mul(3600)
            .and_then(|h| minutes.checked_mul(60).and_then(|m| h.checked_add(m)))
            .ok_or_else(|| DurationError::OutOfRange(raw.to_string()))?;

        return Ok(Duration::from_secs(seconds));
    }

    let idx = input
        .find(|c: char| c.is_ascii_alphabetic())
        .unwrap_or(input.len());
    let (number_part, unit_part) = input.split_at(idx);
    let normalized = number_part.trim().replace(',', ".");

    if normalized.is_empty()
        || !normalized.chars().all(|c| c.is_ascii_digit() || c == '.')
        || normalized.matches('.').count() > 1
    {
        return Err(DurationError::InvalidFormat(raw.to_string()));
    }

    let value: f64 = normalized
        .parse()
        .map_err(|_| DurationError::InvalidFormat(raw.to_string()))?;
    if !value.is_finite() || value < 0.0 {
        return Err(DurationError::InvalidFormat(raw.to_string()));
    }

    let unit = match unit_part.trim().to_ascii_lowercase().as_str() {
        "" => default_unit,
        "s" | "sec" | "secs" | "second" | "seconds" => Unit::Seconds,
        "m" | "min" | "mins" | "minute" | "minutes" => Unit::Minutes,
        "h" | "hr" | "hrs" | "hour" | "hours" => Unit::Hours,
        _ => return Err(DurationError::InvalidFormat(raw.to_string())),
    };

    let multiplier = match unit {
        Unit::Seconds => 1.0,
        Unit::Minutes => 60.0,
        Unit::Hours => 3600.0,
    };

    let total_seconds = value * multiplier;

    Duration::try_from_secs_f64(total_seconds)
        .map_err(|_| DurationError::OutOfRange(raw.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_durations() {
        let cases = [
            ("90", Unit::Minutes, Duration::from_secs(5400)),
            ("90", Unit::Hours, Duration::from_secs(324_000)),
            (" 30 ", Unit::Minutes, Duration::from_secs(1800)),
            ("0", Unit::Minutes, Duration::from_secs(0)),
            ("0.25", Unit::Minutes, Duration::from_secs(15)),
            ("1,5", Unit::Hours, Duration::from_secs(5400)),
            ("1,5 h", Unit::Minutes, Duration::from_secs(5400)),
            ("1.5h", Unit::Minutes, Duration::from_secs(5400)),
            ("1.5 hrs", Unit::Minutes, Duration::from_secs(5400)),
            ("1.5 Hours", Unit::Minutes, Duration::from_secs(5400)),
            ("45 min", Unit::Hours, Duration::from_secs(2700)),
            ("45m", Unit::Hours, Duration::from_secs(2700)),
            ("45 MIN", Unit::Hours, Duration::from_secs(2700)),
            ("2700s", Unit::Hours, Duration::from_secs(2700)),
            ("1:30", Unit::Minutes, Duration::from_secs(5400)),
            ("0:15", Unit::Minutes, Duration::from_secs(900)),
        ];

        for (input, default_unit, expected_duration) in cases {
            let result = parse_duration(input, default_unit);

            assert_eq!(
                result.unwrap(),
                expected_duration,
                "Failed for input: {input}"
            );
        }
    }

    #[test]
    fn test_invalid_durations() {
        let cases = [
            "",
            " ",
            "-5",
            "abc",
            "1:75",
            "1.5.2",
            "NaN",
            "inf",
            "5 parsecs",
            "1:",
            ":30",
            "1e3",
            "+1:30",
        ];

        for input in cases {
            let result = parse_duration(input, Unit::Minutes);

            assert!(result.is_err(), "Expected an error for input: {input}");
        }
    }

    #[test]
    fn test_error_contains_original_input() {
        let result = parse_duration("1:75", Unit::Minutes);

        let error = result.unwrap_err();

        assert!(error.to_string().contains("1:75"));
    }

    #[test]
    fn test_negative_duration() {
        let result = parse_duration("-5", Unit::Minutes);

        assert!(matches!(result, Err(DurationError::NegativeValue(_))));
    }
}
