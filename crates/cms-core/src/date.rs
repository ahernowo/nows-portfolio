//! A tiny calendar date type.
//!
//! We avoid a date library on purpose: Cloudflare Workers have no system
//! clock API in Rust (`SystemTime::now()` panics in WebAssembly), so callers
//! pass Unix seconds in and we do the calendar maths here. The conversion uses
//! Howard Hinnant's well-known `civil_from_days` algorithm.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    /// The UTC calendar date for a Unix timestamp in seconds.
    pub fn from_unix(secs: i64) -> Self {
        let days = secs.div_euclid(86_400);
        civil_from_days(days)
    }

    /// Parse `YYYY-MM-DD`. Returns `None` for anything else, including
    /// impossible dates such as 2026-02-30.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let bytes = s.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let year: i32 = s[0..4].parse().ok()?;
        let month: u8 = s[5..7].parse().ok()?;
        let day: u8 = s[8..10].parse().ok()?;
        if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Dates travel as `"YYYY-MM-DD"` strings in JSON.
impl serde::Serialize for Date {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// RFC 3339 UTC timestamp, for example `2026-10-23T09:41:00Z`.
pub fn rfc3339(secs: i64) -> String {
    let date = Date::from_unix(secs);
    let rem = secs.rem_euclid(86_400);
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        date,
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        _ => 28,
    }
}

fn civil_from_days(days: i64) -> Date {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let year = if m <= 2 { y + 1 } else { y };
    Date {
        year: year as i32,
        month: m as u8,
        day: d as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch() {
        assert_eq!(Date::from_unix(0).to_string(), "1970-01-01");
    }

    #[test]
    fn known_dates() {
        // 2026-10-23T00:00:00Z
        assert_eq!(Date::from_unix(1_792_713_600).to_string(), "2026-10-23");
        // Leap day 2024-02-29T12:00:00Z
        assert_eq!(Date::from_unix(1_709_208_000).to_string(), "2024-02-29");
        // Before 1970
        assert_eq!(Date::from_unix(-86_400).to_string(), "1969-12-31");
    }

    #[test]
    fn rfc3339_format() {
        assert_eq!(rfc3339(1_792_748_460), "2026-10-23T09:41:00Z");
    }

    #[test]
    fn parsing() {
        assert_eq!(
            Date::parse("2026-10-23").map(|d| d.to_string()).as_deref(),
            Some("2026-10-23")
        );
        assert!(Date::parse("2026-02-30").is_none());
        assert!(Date::parse("2024-02-29").is_some());
        assert!(Date::parse("2023-02-29").is_none());
        assert!(Date::parse("23-10-2026").is_none());
        assert!(Date::parse("2026-13-01").is_none());
    }
}
