use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Country {
    E164, // Fallback when only E.164 can be asserted
    US,
    BR,
}

pub trait PhoneLike {
    fn validate(&self) -> ResultApp<()>;
    fn format(&self) -> String; // E.164 normalized
}

#[derive(Debug, Clone)]
pub struct Phone {
    country: Country,
    // Always store the normalized E.164 representation (with leading '+')
    e164: String,
}

impl Phone {
    // Create from raw input, auto-detecting country pattern and normalizing to E.164
    pub fn new(raw: String) -> ResultApp<Self> {
        let trimmed = raw.trim();

        // If already valid E.164, accept as-is
        if is_e164(trimmed) {
            return Ok(Self {
                country: Country::E164,
                e164: trimmed.to_string(),
            });
        }

        // Try US normalizations
        if let Some(e164) = normalize_us(trimmed) {
            return Ok(Self {
                country: Country::US,
                e164,
            });
        }

        // Try BR normalizations
        if let Some(e164) = normalize_br(trimmed) {
            return Ok(Self {
                country: Country::BR,
                e164,
            });
        }

        // Try to coerce digits into E.164 if it looks like it already includes a country code
        if let Some(e164) = digits_to_e164_guess(trimmed) {
            return Ok(Self {
                country: Country::E164,
                e164,
            });
        }

        Err(Arc::new(AppError::Validation(ErrorData::new(
            "invalid-phone",
            "Invalid or unsupported phone format",
        ))))
    }

    pub fn validate(&self) -> ResultApp<()> {
        if is_e164(&self.e164) {
            return Ok(());
        }
        Err(Arc::new(AppError::Validation(ErrorData::new(
            "invalid-phone",
            "Invalid or unsupported phone format",
        ))))
    }

    pub fn value(&self) -> String {
        self.e164.clone()
    }

    pub fn country(&self) -> Country {
        self.country
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e164)
    }
}

fn is_e164(s: &str) -> bool {
    // E.164 allows up to 15 digits, starting with non-zero. We require leading '+'.
    let re = Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
    re.is_match(s)
}

fn digits_only(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}

// US normalization: accept common forms and output +1XXXXXXXXXX
fn normalize_us(s: &str) -> Option<String> {
    // Extract digits
    let digits = digits_only(s);
    // Expect 10 digits (NPA-NXX-XXXX) or 11 with leading 1
    let (cc, rest) = if digits.len() == 10 {
        ("1", digits)
    } else if digits.len() == 11 && digits.starts_with('1') {
        ("1", digits[1..].to_string())
    } else {
        return None;
    };
    // Basic NANP sanity: first digit of area code and exchange cannot be 0 or 1
    let ac_first = rest.chars().next()?;
    let ex_first = rest.chars().nth(3)?;
    if ac_first < '2' || ex_first < '2' {
        return None;
    }
    Some(format!("+{}{}", cc, rest))
}

// Brazil normalization: accept common forms and output +55AA9XXXXXXXX or +55AAXXXXXXXX (landline)
fn normalize_br(s: &str) -> Option<String> {
    let mut digits = digits_only(s);
    // Remove leading country code 55 if present
    if digits.starts_with("55") {
        digits = digits[2..].to_string();
    }
    // Now expect DD + number; mobile is 11 digits (9 + 8), landline often 10 digits
    if digits.len() == 11 {
        // 2-digit area + 9-digit mobile
        return Some(format!("+55{}", digits));
    } else if digits.len() == 10 {
        // landline
        return Some(format!("+55{}", digits));
    }
    None
}

// If it already contains a plausible country code and total digits 8..15, accept as E.164 with '+'
fn digits_to_e164_guess(s: &str) -> Option<String> {
    let digits = digits_only(s);
    if digits.len() < 8 || digits.len() > 15 {
        return None;
    }
    // Prepend '+' if missing and ensure first digit isn't 0
    if digits.starts_with('0') {
        return None;
    }
    Some(format!("+{}", digits))
}
