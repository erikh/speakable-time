mod loader;

#[cfg(feature = "translation")]
use self::loader::*;
use crate::enums::Words;
#[cfg(not(feature = "translation"))]
use crate::translation_map;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(feature = "translation")]
lazy_static::lazy_static! {
    /// Locale-aware default translation. Pulls the best definition it can find in the binary on boot.
    pub static ref DEFAULT_TRANSLATION: Translator<'static> = crate::load_locale!(sys_locale::get_locale().unwrap_or(String::from("en-US")));
}

#[cfg(not(feature = "translation"))]
lazy_static::lazy_static! {
    /// The standard loaded translation that is created to provide some form of translation. It is
    /// standard to American English. If you wish to use other languages, please provided your own
    /// [Translator].
    pub static ref DEFAULT_TRANSLATION: Translator<'static> = Translator::new(translation_map!(
            (Words::January, "January"),
            (Words::February, "February"),
            (Words::March, "March"),
            (Words::April, "April"),
            (Words::May, "May"),
            (Words::June, "June"),
            (Words::July, "July"),
            (Words::August, "August"),
            (Words::September, "September"),
            (Words::October, "October"),
            (Words::November, "November"),
            (Words::December, "December"),
            (Words::Suffix(0), "th"),
            (Words::Suffix(1), "st"),
            (Words::Suffix(2), "nd"),
            (Words::Suffix(3), "rd"),
            (Words::Suffix(4), "th"),
            (Words::Suffix(5), "th"),
            (Words::Suffix(6), "th"),
            (Words::Suffix(7), "th"),
            (Words::Suffix(8), "th"),
            (Words::Suffix(9), "th"),
            (Words::Noon, "Noon"),
            (Words::Midnight, "Midnight"),
            (Words::PM, "PM"),
            (Words::AM, "AM"),
            (Words::A, "a"),
            (Words::In, "in"),
            (Words::An, "an"),
            (Words::At, "at"),
            (Words::Ago, "ago"),
            (Words::Last, "last"),
            (Words::Year, "year"),
            (Words::Week, "week"),
            (Words::Month, "month"),
            (Words::Day, "day"),
            (Words::Hour, "hour"),
            (Words::Minute, "minute"),
            (Words::Second, "second"),
            (Words::YearPlural, "years"),
            (Words::WeekPlural, "weeks"),
            (Words::MonthPlural, "months"),
            (Words::DayPlural, "days"),
            (Words::HourPlural, "hours"),
            (Words::MinutePlural, "minutes"),
            (Words::SecondPlural, "seconds"),
            (Words::Yesterday, "Yesterday"),
            (Words::Today, "Today"),
            (Words::Tomorrow, "Tomorrow"),
            (Words::Sunday, "Sunday"),
            (Words::Monday, "Monday"),
            (Words::Tuesday, "Tuesday"),
            (Words::Wednesday, "Wednesday"),
            (Words::Thursday, "Thursday"),
            (Words::Friday, "Friday"),
            (Words::Saturday, "Saturday"),
            (Words::And, "and"),
            (Words::FromNow, "from now")
    ));
}

/// Map of translation [Words] to their literal meanings.
pub type TranslationMap<'a> = HashMap<Words, &'a str>;

/// The [Translator] provides a format string conversion system that is independent of
/// strftime/strptime (which means they can both be used) but also is generic enough that it can
/// incorporate whole translation tables. It is expected that
/// [FormatGenerator](crate::approximate::format_generator::FormatGenerator) implementations will
/// generate these formats and the translator will just swap out the right terms.
///
/// If the translation feature is enabled, translations will automatically load based on the user's
/// locale. See [load_locale](crate::load_locale) for more information.
pub struct Translator<'a> {
    map: TranslationMap<'a>,
}

impl<'a> Translator<'a> {
    /// Construct a new translator from a [TranslationMap].
    pub fn new(map: TranslationMap<'a>) -> Self {
        Self { map }
    }

    /// Given a word, translate it to the literal meaning, if it exists. Otherwise, return [None].
    #[inline]
    pub fn translate(&self, s: &Words) -> Option<&'a str> {
        self.map.get(s).copied()
    }

    /// Given a format, parse it and return the literal meaning. Formats start with %{, contain a
    /// term, and end in }. If you need to include a %, use %%. Braces may be used anywhere
    /// outside of the % syntax, but you may not use more than one { in a row before completing it
    /// with a }, and the string must be fully closed.
    ///
    /// If any of these terms are violated, you will get an [Err]. Otherwise, your string will be
    /// translated.
    pub fn format(&self, format: &str) -> Result<String> {
        let mut in_match = false;
        let mut in_brace = false;

        let mut s = String::new();
        let mut cap = String::new();

        for ch in format.chars() {
            match ch {
                '%' => {
                    if in_match && !in_brace {
                        s.push(ch);
                        in_match = false;
                    } else {
                        if in_brace || in_match {
                            return Err(anyhow!("Invalid format (format attempted within format)"));
                        }

                        in_match = true;
                        in_brace = false;
                    }
                }
                '{' => {
                    if !in_match {
                        s.push(ch);
                    } else {
                        if in_brace {
                            return Err(anyhow!(
                                "Invalid format (open brace attempted within open brace)"
                            ));
                        }

                        in_brace = true;
                    }
                }
                '}' => {
                    if !in_match {
                        s.push(ch);
                    } else {
                        if !in_brace {
                            return Err(anyhow!(
                                "Invalid format (close brace attempted outside open brace)"
                            ));
                        }

                        in_brace = false;
                        in_match = false;

                        if let Some(word) = &Words::from_str(&cap).ok() {
                            s += self.translate(&word).unwrap_or_default();
                        }

                        cap = String::new();
                    }
                }
                _ => {
                    if in_brace && in_match {
                        cap.push(ch);
                    } else {
                        s.push(ch);
                    }
                }
            }
        }

        if in_brace {
            return Err(anyhow!("Invalid format (unclosed brace)"));
        }

        if in_match {
            return Err(anyhow!("Invalid format (incomplete match)"));
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_translate() {
        use super::DEFAULT_TRANSLATION;
        use crate::enums::Words;

        assert_eq!(
            "Yesterday",
            DEFAULT_TRANSLATION.translate(&Words::Yesterday).unwrap()
        );
    }

    #[test]
    fn test_format() {
        use super::DEFAULT_TRANSLATION;

        assert_eq!(
            "Yesterday at Noon",
            DEFAULT_TRANSLATION
                .format("%{yesterday} %{at} %{noon}")
                .unwrap()
        );
        assert_eq!("", DEFAULT_TRANSLATION.format("%{poop}").unwrap());
        assert_eq!("", DEFAULT_TRANSLATION.format("%{}").unwrap());
        assert_eq!("%", DEFAULT_TRANSLATION.format("%%").unwrap());
        assert_eq!("{", DEFAULT_TRANSLATION.format("{").unwrap());
        assert_eq!("}", DEFAULT_TRANSLATION.format("}").unwrap());
        assert!(DEFAULT_TRANSLATION.format("%").is_err());
        assert!(DEFAULT_TRANSLATION.format("%%%").is_err());
        assert!(DEFAULT_TRANSLATION.format("%{").is_err());
        assert!(DEFAULT_TRANSLATION.format("%}").is_err());
    }
}
