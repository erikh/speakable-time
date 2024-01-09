mod words;
pub use self::words::Words;

/// A list of months in order. They are numerically indexed, string indexed and can be
/// cross-converted to several other types.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Into<Month> for u32 {
    fn into(self) -> Month {
        match self {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => panic!("Invalid Month"),
        }
    }
}

impl Into<Words> for Month {
    fn into(self) -> Words {
        match self {
            Self::January => Words::January,
            Self::February => Words::February,
            Self::March => Words::March,
            Self::April => Words::April,
            Self::May => Words::May,
            Self::June => Words::June,
            Self::July => Words::July,
            Self::August => Words::August,
            Self::September => Words::September,
            Self::October => Words::October,
            Self::November => Words::November,
            Self::December => Words::December,
        }
    }
}

impl Into<Option<Month>> for Words {
    fn into(self) -> Option<Month> {
        match self {
            Self::January => Some(Month::January),
            Self::February => Some(Month::February),
            Self::March => Some(Month::March),
            Self::April => Some(Month::April),
            Self::May => Some(Month::May),
            Self::June => Some(Month::June),
            Self::July => Some(Month::July),
            Self::August => Some(Month::August),
            Self::September => Some(Month::September),
            Self::October => Some(Month::October),
            Self::November => Some(Month::November),
            Self::December => Some(Month::December),
            _ => None,
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Month::January => "january",
            Month::February => "february",
            Month::March => "march",
            Month::April => "april",
            Month::May => "may",
            Month::June => "june",
            Month::July => "july",
            Month::August => "august",
            Month::September => "september",
            Month::October => "october",
            Month::November => "november",
            Month::December => "december",
        };

        f.write_str(s)
    }
}

/// A list of weekdays in order, starting with Sunday. They are numerically indexed, string indexed and can be
/// cross-converted to several other types.
///
/// NOTE: Chrono days start with Monday, so conversions from u32 will index 0 as Monday. This will
/// be resolved eventually.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<chrono::Weekday> for Weekday {
    fn from(value: chrono::Weekday) -> Self {
        match value {
            chrono::Weekday::Mon => Weekday::Monday,
            chrono::Weekday::Tue => Weekday::Tuesday,
            chrono::Weekday::Wed => Weekday::Wednesday,
            chrono::Weekday::Thu => Weekday::Thursday,
            chrono::Weekday::Fri => Weekday::Friday,
            chrono::Weekday::Sat => Weekday::Saturday,
            chrono::Weekday::Sun => Weekday::Sunday,
        }
    }
}

impl Into<Weekday> for u32 {
    fn into(self) -> Weekday {
        match self {
            // chrono days start with monday
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            6 => Weekday::Sunday,
            _ => panic!("Invalid Weekday"),
        }
    }
}

impl Into<Words> for Weekday {
    fn into(self) -> Words {
        match self {
            Self::Sunday => Words::Sunday,
            Self::Monday => Words::Monday,
            Self::Tuesday => Words::Tuesday,
            Self::Wednesday => Words::Wednesday,
            Self::Thursday => Words::Thursday,
            Self::Friday => Words::Friday,
            Self::Saturday => Words::Saturday,
        }
    }
}

impl Into<Option<Weekday>> for Words {
    fn into(self) -> Option<Weekday> {
        match self {
            Self::Sunday => Some(Weekday::Sunday),
            Self::Monday => Some(Weekday::Monday),
            Self::Tuesday => Some(Weekday::Tuesday),
            Self::Wednesday => Some(Weekday::Wednesday),
            Self::Thursday => Some(Weekday::Thursday),
            Self::Friday => Some(Weekday::Friday),
            Self::Saturday => Some(Weekday::Saturday),
            _ => None,
        }
    }
}

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Weekday::Sunday => "sunday",
            Weekday::Monday => "monday",
            Weekday::Tuesday => "tuesday",
            Weekday::Wednesday => "wednesday",
            Weekday::Thursday => "thursday",
            Weekday::Friday => "friday",
            Weekday::Saturday => "saturday",
        };

        f.write_str(s)
    }
}
