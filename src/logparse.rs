/// Functions related to parsing pacman logs
use std::collections::BTreeMap;

use chrono::DateTime;
use memchr::{memchr_iter, memmem};

#[derive(Debug, PartialEq)]
pub enum LogEvent {
    TransactionStarted,
    TransactionCompleted,
    Installed {
        package: String,
        version: String,
    },
    Removed {
        package: String,
        version: String,
    },
    Upgraded {
        package: String,
        old_version: String,
        new_version: String,
    },
    Downgraded {
        package: String,
        old_version: String,
        new_version: String,
    },
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to parse timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Failed to parse package info: {0}")]
    InvalidPackageInfo(String),

    #[error("Invalid UTF-8 in log line")]
    InvalidUtf8(#[from] std::str::Utf8Error),
}

pub fn parse_log(content: &str) -> Result<BTreeMap<i64, Vec<LogEvent>>, ParseError> {
    let bytes = content.as_bytes();
    let mut events: BTreeMap<i64, Vec<LogEvent>> = BTreeMap::new();

    let alpm_finder = memmem::Finder::new(b"[ALPM] ");
    let tx_started = memmem::Finder::new(b"transaction started");
    let tx_completed = memmem::Finder::new(b"transaction completed");
    let installed = memmem::Finder::new(b"installed ");
    let removed = memmem::Finder::new(b"removed ");
    let upgraded = memmem::Finder::new(b"upgraded ");
    let downgraded = memmem::Finder::new(b"downgraded ");
    let warning = memmem::Finder::new(b"warning: ");

    let mut line_start = 0;
    for line_end in memchr_iter(b'\n', bytes) {
        let line = &bytes[line_start..line_end];

        // We only care about lines containing [ALPM]
        if let Some(alpm_pos) = alpm_finder.find(line) {
            let timestamp = parse_timestamp(&line[..alpm_pos])?;
            let after_alpm = &line[alpm_pos + 7..];

            let event = if tx_started.find(after_alpm).is_some() {
                Some(LogEvent::TransactionStarted)
            } else if tx_completed.find(after_alpm).is_some() {
                Some(LogEvent::TransactionCompleted)
            } else if warning.find(after_alpm).is_some() {
                None // skip for now, but messes with other matchers
            } else if let Some(pos) = upgraded.find(after_alpm) {
                Some(parse_upgrade(&after_alpm[pos + 9..])?)
            } else if let Some(pos) = installed.find(after_alpm) {
                Some(parse_installed(&after_alpm[pos + 10..])?)
            } else if let Some(pos) = removed.find(after_alpm) {
                Some(parse_removed(&after_alpm[pos + 8..])?)
            } else if let Some(pos) = downgraded.find(after_alpm) {
                Some(parse_downgrade(&after_alpm[pos + 11..])?)
            } else {
                None
            };

            if let Some(event) = event {
                events.entry(timestamp).or_default().push(event);
            }
        }

        line_start = line_end + 1;
    }

    Ok(events)
}

// pacman.log uses timestamps in the format: [2026-01-28T19:36:35-0800]
fn parse_timestamp(bytes: &[u8]) -> Result<i64, ParseError> {
    // 26 + whitepsace at the end of ]
    if bytes.len() < 27 || bytes[0] != b'[' {
        return Err(ParseError::InvalidTimestamp(
            String::from_utf8_lossy(bytes).to_string(),
        ));
    }

    let end = memchr::memchr(b']', bytes)
        .ok_or_else(|| ParseError::InvalidTimestamp(String::from_utf8_lossy(bytes).to_string()))?;

    let timestamp_str = std::str::from_utf8(&bytes[1..end])?;
    DateTime::parse_from_str(timestamp_str, "%Y-%m-%dT%H:%M:%S%z")
        .map(|dt| dt.timestamp())
        .map_err(|e| ParseError::InvalidTimestamp(format!("{}: {}", timestamp_str, e)))
}

fn parse_installed(bytes: &[u8]) -> Result<LogEvent, ParseError> {
    let s = std::str::from_utf8(bytes)?;
    let (pkg, ver) = parse_package_version(s)?;
    Ok(LogEvent::Installed {
        package: pkg.to_string(),
        version: ver.to_string(),
    })
}

fn parse_removed(bytes: &[u8]) -> Result<LogEvent, ParseError> {
    let s = std::str::from_utf8(bytes)?;
    let (pkg, ver) = parse_package_version(s)?;
    Ok(LogEvent::Removed {
        package: pkg.to_string(),
        version: ver.to_string(),
    })
}

// Upgrade looks like this: gelly (4.0.6-10 -> 4.1.0-1)
fn parse_upgrade(bytes: &[u8]) -> Result<LogEvent, ParseError> {
    let s = std::str::from_utf8(bytes)?;
    let open = s
        .find('(')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    let arrow = s
        .find(" -> ")
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    let close = s
        .find(')')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;

    Ok(LogEvent::Upgraded {
        package: s[..open].trim().to_string(),
        old_version: s[open + 1..arrow].to_string(),
        new_version: s[arrow + 4..close].to_string(),
    })
}

// Downgrade looks like this: linux (6.14.7.arch2-1 -> 6.14.6.arch1-1)
fn parse_downgrade(bytes: &[u8]) -> Result<LogEvent, ParseError> {
    let s = std::str::from_utf8(bytes)?;
    let open = s
        .find('(')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    let arrow = s
        .find(" -> ")
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    let close = s
        .find(')')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;

    Ok(LogEvent::Downgraded {
        package: s[..open].trim().to_string(),
        old_version: s[open + 1..arrow].to_string(),
        new_version: s[arrow + 4..close].to_string(),
    })
}

// version looks like: gelly (0.3.0-1)
fn parse_package_version(s: &str) -> Result<(&str, &str), ParseError> {
    let open = s
        .find('(')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    let close = s
        .find(')')
        .ok_or_else(|| ParseError::InvalidPackageInfo(s.to_string()))?;
    Ok((s[..open].trim(), &s[open + 1..close]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_version() {
        assert_eq!(
            parse_package_version("bar (1.2.3-4)"),
            Ok(("bar", "1.2.3-4"))
        );
    }

    #[test]
    fn test_parse_upgraded() {
        assert_eq!(
            parse_upgrade("gelly (4.0.6-10 -> 4.1.0-1)".as_bytes()),
            Ok(LogEvent::Upgraded {
                package: "gelly".to_string(),
                old_version: "4.0.6-10".to_string(),
                new_version: "4.1.0-1".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_downgraded() {
        assert_eq!(
            parse_downgrade("linux (6.14.7.arch2-1 -> 6.14.6.arch1-1)".as_bytes()),
            Ok(LogEvent::Downgraded {
                package: "linux".to_string(),
                old_version: "6.14.7.arch2-1".to_string(),
                new_version: "6.14.6.arch1-1".to_string(),
            })
        );
    }
}
