use super::enums::Words;
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
        let abs = crate::absolute_duration(dt, dt2);
        match self {
            Self::Second => abs.num_seconds() < 60,
            Self::Minute => abs.num_minutes() > 1 && abs.num_hours() == 0,
            Self::Hour => abs.num_hours() > 1 && abs.num_days() == 0,
            Self::Day => abs.num_days() > 1 && abs.num_days() < 30,
            Self::Week => abs.num_weeks() > 1 && abs.num_days() < 30,
            Self::Month => abs.num_days() > 30 && abs.num_days() < 365,
            Self::Year => abs.num_days() > 365,
        }
    }
}
