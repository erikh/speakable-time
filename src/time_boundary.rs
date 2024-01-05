use super::enums::{Words, DAY, HOUR, MINUTE, MONTH, WEEK, YEAR};
use chrono::prelude::*;

/// Time boundaries denote changes in the type of time, from a series of seconds to a series of
/// minutes to a series of hours crosses several time boundaries. This enum organizes those
/// boundaries into units which can then be changed around and compared.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum TimeBoundary {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl Into<Words> for TimeBoundary {
    fn into(self) -> Words {
        match self {
            Self::Second => Words::Second,
            Self::Minute => Words::Minute,
            Self::Hour => Words::Hour,
            Self::Day => Words::Day,
            Self::Week => Words::Week,
            Self::Month => Words::Month,
            Self::Year => Words::Year,
        }
    }
}

impl Into<Option<TimeBoundary>> for Words {
    fn into(self) -> Option<TimeBoundary> {
        match self {
            Self::Second => Some(TimeBoundary::Second),
            Self::Minute => Some(TimeBoundary::Minute),
            Self::Hour => Some(TimeBoundary::Hour),
            Self::Day => Some(TimeBoundary::Day),
            Self::Week => Some(TimeBoundary::Week),
            Self::Month => Some(TimeBoundary::Month),
            Self::Year => Some(TimeBoundary::Year),
            _ => None,
        }
    }
}

impl TimeBoundary {
    /// Yield all time boundaries in order, with the most significant (years) first.
    pub fn all() -> [Self; 7] {
        [
            Self::Year,
            Self::Month,
            Self::Week,
            Self::Day,
            Self::Hour,
            Self::Minute,
            Self::Second,
        ]
    }

    /// Yield the highest boundary the difference between these two times can report
    #[inline]
    pub fn highest(dt: DateTime<Local>, dt2: DateTime<Local>) -> Option<Self> {
        for item in Self::all() {
            if item.within(dt, dt2) {
                return Some(item);
            }
        }

        None
    }

    /// Get the specific numeric value for the current boundary.
    #[inline]
    pub fn value(&self, dt: DateTime<Local>) -> u32 {
        match self {
            Self::Second => dt.second(),
            Self::Minute => dt.minute(),
            Self::Hour => dt.hour(),
            Self::Day => dt.day(),
            Self::Week => dt.iso_week().week(),
            Self::Month => dt.month(),
            Self::Year => dt.year().try_into().unwrap(),
        }
    }

    /// Does the difference of these two times span this boundary?
    #[inline]
    pub fn within(&self, dt: DateTime<Local>, dt2: DateTime<Local>) -> bool {
        let abs = crate::absolute_duration(dt, dt2).num_seconds();
        match self {
            Self::Second => abs < MINUTE,
            Self::Minute => abs > MINUTE && abs < HOUR,
            Self::Hour => abs > HOUR && abs < DAY,
            Self::Day => abs > DAY && abs < MONTH,
            Self::Week => abs > WEEK && abs % WEEK < DAY,
            Self::Month => abs > MONTH && abs % MONTH < WEEK,
            Self::Year => abs > YEAR && abs % YEAR < MONTH,
        }
    }
}
