//! Speakable time in this case, are time periods or intervals that are used informally, times
//! like "10 years ago" or "next week". This library aims to deconstruct times in a way that they
//! can be treated like a grammar for generating human-friendly times.
//!
//! To accomplish this, we seed an [Approximator] with a list of selectors called
//! [ApproximateFilter]s, and a
//! [FormatGenerator](crate::approximate::format_generator::FormatGenerator), of which there are
//! currently two styles: [CoarseRoundFormat] and [FancyDurationFormat]. The approximator computes
//! durations into tokenized parts (called [ApproximateState](crate::approximate::ApproximateState)) which are picked out by the
//! [ApproximateFilter]s, which are then fed to the formatter of choice to generate a syntax. Then,
//! a [Translator] is used to convert that final syntax into something you can read. It is fully
//! localized and flexible.
//!
//! There a few macros to make this easier. If the `translation` feature is enabled, it will allow
//! you to provide translation maps as YAML files that live in your build directory. It will then
//! select the right mapping to use for a given locale automatically on boot.
//!
//! There are plenty of docs for writing your own formatter, too. Just see
//! [FormatGenerator](crate::approximate::format_generator::FormatGenerator).
//!
//!```
//!  use speakable_time::prelude::*;
//!
//!  let approx = approximator!(
//!      CoarseRoundFormat::default(),
//!      ApproximateFilter::Round(TimeBoundary::Day),
//!      ApproximateFilter::Relative
//!  );
//!
//!  let dt = chrono::Local::now() - chrono::Duration::days(2);
//!
//!  assert_eq!("2 days ago", from_now!(dt, approx).unwrap());
//!
//!  let approx = approximator!(
//!      CoarseRoundFormat::default(),
//!      ApproximateFilter::Round(TimeBoundary::Year),
//!      ApproximateFilter::Relative
//!  );
//!
//!  let dt2 = chrono::Local::now() - chrono::Duration::days(46 * 365); // account for leap
//!                                                                     // years
//!  assert_eq!("45 years ago", time_diff!(dt2, dt, approx).unwrap());
//!
//!  let approx = approximator!(
//!      FancyDurationFormat::default(),
//!      ApproximateFilter::TopRounds(1),
//!      ApproximateFilter::Relative
//!  );
//!
//!  assert_eq!("45y ago", time_diff!(dt2, dt, approx).unwrap());
//!
//!  assert_eq!(
//!      "in 2d",
//!      time_diff!(
//!          chrono::Local::now() + chrono::Duration::days(2) + chrono::Duration::seconds(1),
//!          chrono::Local::now(),
//!          approx
//!      ).unwrap()
//!  );
//!```

/// Compute relative durations based on a combination of rules; generate a grammar
pub mod approximate;
/// Enums we use throughout the library
pub mod enums;
/// Duration scoping done with relative intervals
pub mod time_boundary;
/// Translation engine supplying translated literals external to grammar
pub mod translator;

pub use crate::{
    approximate::{ApproximateFilter, Approximator, CoarseRoundFormat, FancyDurationFormat},
    enums::Words,
    time_boundary::TimeBoundary,
    translator::{TranslationMap, Translator, DEFAULT_TRANSLATION},
};

/// Use this to import major chunks of functionality from speakable_time.
pub mod prelude {
    pub use crate::{
        approximator, from_now, time_diff, translation_map, translator, ApproximateFilter,
        CoarseRoundFormat, FancyDurationFormat, TimeBoundary, Words,
    };
}

/// Build an [Approximator] easily. This takes a
/// [FormatGenerator](self::approximate::format_generator::FormatGenerator) and a list of
/// [ApproximateFilter]s to build one, saving you a little typing.
#[macro_export]
macro_rules! approximator {
    ($format:expr, $($filter:expr),*) => {{
        use $crate::approximate::Approximator;
        Approximator::new(vec![$($filter,)*], $format)
    }}
}

/// Build a [Translator]. Translators can be used to keep static sematics with
/// translated literals.
#[macro_export]
macro_rules! translator {
    ($(($key:expr, $value:expr)),*) => {{
        use $crate::translator::Translator;
        Translator::new(map!($(($key, $value),)*))
    }}
}

/// Build a string which is the formatted result of a time, provided, subtracted from the current
/// time, and run through the approximation engine as well as any translation.
#[macro_export]
macro_rules! from_now {
    ($dt:expr, $approx:expr, $translation:expr) => {{
        $translation.format(&$approx.from_now($dt).to_string())
    }};
    ($dt:expr, $approx:expr) => {{
        use $crate::translator::DEFAULT_TRANSLATION;
        DEFAULT_TRANSLATION.format(&$approx.from_now($dt).to_string())
    }};
}

/// Similar to [from_now], this computes the results with from subtracted from to with the
/// same arguments otherwise. A final formatted string is produced.
#[macro_export]
macro_rules! time_diff {
    ($from:expr, $to:expr, $approx:expr, $translation:expr) => {{
        $translation.format(&$approx.difference($from, $to).to_string())
    }};
    ($from:expr, $to:expr, $approx:expr) => {{
        use $crate::translator::DEFAULT_TRANSLATION;
        DEFAULT_TRANSLATION.format(&$approx.difference($from, $to).to_string())
    }};
}

/// This is a simple macro for creating [HashMap](std::collections::HashMap)s from paired tuples used to
/// carry the (key, value) data. The tuples are inserted in order, so collisions will always be
/// last write wins.
#[macro_export]
macro_rules! translation_map {
    ($(($key:expr, $value:expr)),*) => {{
        let mut h = std::collections::HashMap::default();

        for (key, value) in [$(($key, $value),)*] {
            h.insert(key, value);
        }

        h
    }}
}

fn absolute_duration(
    dt1: chrono::DateTime<chrono::Local>,
    dt2: chrono::DateTime<chrono::Local>,
) -> chrono::Duration {
    (dt1 - dt2).abs()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_translation_map() {
        use std::collections::HashMap;

        let h: HashMap<&str, usize> = translation_map!(("one", 1), ("two", 2));
        assert_eq!(h["one"], 1);
        assert_eq!(h["two"], 2);
    }

    #[test]
    fn test_macros() {
        use crate::prelude::*;

        let approx = approximator!(
            CoarseRoundFormat::default(),
            ApproximateFilter::Round(TimeBoundary::Day),
            ApproximateFilter::Relative
        );

        let dt = chrono::Local::now() - chrono::Duration::days(2);

        assert_eq!("2 days ago", from_now!(dt, approx).unwrap());

        let approx = approximator!(
            CoarseRoundFormat::default(),
            ApproximateFilter::Round(TimeBoundary::Year),
            ApproximateFilter::Relative
        );

        let dt2 = chrono::Local::now() - chrono::Duration::days(46 * 365); // account for leap
                                                                           // years
        assert_eq!("45 years ago", time_diff!(dt2, dt, approx).unwrap());

        let approx = approximator!(
            FancyDurationFormat::default(),
            ApproximateFilter::TopRounds(1),
            ApproximateFilter::Relative
        );

        assert_eq!("45y ago", time_diff!(dt2, dt, approx).unwrap());

        assert_eq!(
            "in 2d",
            time_diff!(
                chrono::Local::now() + chrono::Duration::days(2) + chrono::Duration::seconds(1),
                chrono::Local::now(),
                approx
            )
            .unwrap()
        );
    }
}
