/// A list of words used in the [Translator](crate::translator::Translator). Each one of these
/// corresponds to a format string. The formatters that implement
/// [FormatGenerator](crate::approximate::format_generator::FormatGenerator) generate strings
/// containing these format strings. The [Words] are used to isolate them into tokens for further
/// manipulation and conversion between types.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Words {
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
    Suffix(u8),
    Noon,
    Midnight,
    PM,
    AM,
    A,
    In,
    An,
    And,
    FromNow,
    At,
    Ago,
    Last,
    Year,
    Week,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    YearPlural,
    WeekPlural,
    MonthPlural,
    DayPlural,
    HourPlural,
    MinutePlural,
    SecondPlural,
    Yesterday,
    Today,
    Tomorrow,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Words {
    /// For a given word, make it plural if possible. Otherwise just return the same word.
    pub fn plural(&self) -> Self {
        match self {
            Self::Year => Self::YearPlural,
            Self::Week => Self::WeekPlural,
            Self::Month => Self::MonthPlural,
            Self::Day => Self::DayPlural,
            Self::Hour => Self::HourPlural,
            Self::Minute => Self::MinutePlural,
            Self::Second => Self::SecondPlural,
            _ => self.clone(),
        }
    }
}

impl std::fmt::Display for Words {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Words::January => "january".to_string(),
            Words::February => "february".to_string(),
            Words::March => "march".to_string(),
            Words::April => "april".to_string(),
            Words::May => "may".to_string(),
            Words::June => "june".to_string(),
            Words::July => "july".to_string(),
            Words::August => "august".to_string(),
            Words::September => "september".to_string(),
            Words::October => "october".to_string(),
            Words::November => "november".to_string(),
            Words::December => "december".to_string(),
            Words::Suffix(suffix) => format!("suffix_{}", suffix),
            Words::Noon => "noon".to_string(),
            Words::Midnight => "midnight".to_string(),
            Words::PM => "pm".to_string(),
            Words::AM => "am".to_string(),
            Words::A => "a".to_string(),
            Words::In => "in".to_string(),
            Words::An => "an".to_string(),
            Words::And => "and".to_string(),
            Words::FromNow => "from now".to_string(),
            Words::At => "at".to_string(),
            Words::Ago => "ago".to_string(),
            Words::Last => "last".to_string(),
            Words::Year => "year".to_string(),
            Words::Week => "week".to_string(),
            Words::Month => "month".to_string(),
            Words::Day => "day".to_string(),
            Words::Hour => "hour".to_string(),
            Words::Minute => "minute".to_string(),
            Words::Second => "second".to_string(),
            Words::YearPlural => "years".to_string(),
            Words::WeekPlural => "weeks".to_string(),
            Words::MonthPlural => "months".to_string(),
            Words::DayPlural => "days".to_string(),
            Words::HourPlural => "hours".to_string(),
            Words::MinutePlural => "minutes".to_string(),
            Words::SecondPlural => "seconds".to_string(),
            Words::Yesterday => "yesterday".to_string(),
            Words::Today => "today".to_string(),
            Words::Tomorrow => "tomorrow".to_string(),
            Words::Sunday => "sunday".to_string(),
            Words::Monday => "monday".to_string(),
            Words::Tuesday => "tuesday".to_string(),
            Words::Wednesday => "wednesday".to_string(),
            Words::Thursday => "thursday".to_string(),
            Words::Friday => "friday".to_string(),
            Words::Saturday => "saturday".to_string(),
        };

        f.write_str(&s)
    }
}

impl std::str::FromStr for Words {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "january" => Words::January,
            "february" => Words::February,
            "march" => Words::March,
            "april" => Words::April,
            "may" => Words::May,
            "june" => Words::June,
            "july" => Words::July,
            "august" => Words::August,
            "september" => Words::September,
            "october" => Words::October,
            "november" => Words::November,
            "december" => Words::December,
            "suffix_0" => Words::Suffix(0),
            "suffix_1" => Words::Suffix(1),
            "suffix_2" => Words::Suffix(2),
            "suffix_3" => Words::Suffix(3),
            "suffix_4" => Words::Suffix(4),
            "suffix_5" => Words::Suffix(5),
            "suffix_6" => Words::Suffix(6),
            "suffix_7" => Words::Suffix(7),
            "suffix_8" => Words::Suffix(8),
            "suffix_9" => Words::Suffix(9),
            "noon" => Words::Noon,
            "midnight" => Words::Midnight,
            "pm" => Words::PM,
            "am" => Words::AM,
            "a" => Words::A,
            "in" => Words::In,
            "an" => Words::An,
            "and" => Words::And,
            "from now" => Words::FromNow,
            "at" => Words::At,
            "ago" => Words::Ago,
            "last" => Words::Last,
            "year" => Words::Year,
            "week" => Words::Week,
            "month" => Words::Month,
            "day" => Words::Day,
            "hour" => Words::Hour,
            "minute" => Words::Minute,
            "second" => Words::Second,
            "years" => Words::YearPlural,
            "weeks" => Words::WeekPlural,
            "months" => Words::MonthPlural,
            "days" => Words::DayPlural,
            "hours" => Words::HourPlural,
            "minutes" => Words::MinutePlural,
            "seconds" => Words::SecondPlural,
            "yesterday" => Words::Yesterday,
            "today" => Words::Today,
            "tomorrow" => Words::Tomorrow,
            "sunday" => Words::Sunday,
            "monday" => Words::Monday,
            "tuesday" => Words::Tuesday,
            "wednesday" => Words::Wednesday,
            "thursday" => Words::Thursday,
            "friday" => Words::Friday,
            "saturday" => Words::Saturday,
            x => return Err(anyhow::anyhow!("invalid word '{}'", x)),
        })
    }
}

#[cfg(feature = "translation")]
/// Serde support for [Words] allows us to use it in YAML. See [load_locale](crate::load_locale)
/// for more information.
mod serde {
    use serde::{de::Visitor, Deserialize, Serialize};
    use std::str::FromStr;

    impl Serialize for super::Words {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct WordsVisitor;
    impl Visitor<'_> for WordsVisitor {
        type Value = super::Words;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expecting a speakable-time format word")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match super::Words::from_str(v) {
                Ok(word) => Ok(word),
                Err(e) => Err(E::custom(e.to_string())),
            }
        }
    }
    impl<'de> Deserialize<'de> for super::Words {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(WordsVisitor)
        }
    }
}
