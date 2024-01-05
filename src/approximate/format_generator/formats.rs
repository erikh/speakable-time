use super::FormatGenerator;
use crate::{
    approximate::{ApproximateState, StateCollection},
    enums::Words,
    time_boundary::TimeBoundary,
};

/// This format is based off of the `fancy_duration` crate and generates durations in times like
/// `2y1h15m`.
#[derive(Clone, Default)]
pub struct FancyDurationFormat {
    formats: Vec<ApproximateState>,
    parsed: bool,
}

impl FormatGenerator for FancyDurationFormat {
    fn set_is_parsed(&mut self) {
        self.parsed = true;
    }

    fn is_parsed(&self) -> bool {
        self.parsed
    }

    fn add(&mut self, states: StateCollection) {
        for state in states.0 {
            match state {
                ApproximateState::Value(..) | ApproximateState::InPast(_) => {
                    self.formats.push(state)
                }
                _ => {}
            }
        }
    }

    fn format(&self) -> String {
        let mut in_past: Option<bool> = None;
        let mut s = String::new();

        for format in &self.formats {
            match format {
                ApproximateState::InPast(past) => in_past = Some(*past),
                ApproximateState::Value(relative, time) => {
                    let rs = match relative {
                        TimeBoundary::Year => "y",
                        TimeBoundary::Month => "m",
                        TimeBoundary::Week => "w",
                        TimeBoundary::Day => "d",
                        TimeBoundary::Hour => "h",
                        TimeBoundary::Minute => "m",
                        TimeBoundary::Second => "s",
                    };

                    s += &format!("{}{}", time, rs);
                }
                _ => {}
            }
        }

        if let Some(past) = in_past {
            if past {
                format!("{} %{{ago}}", s)
            } else {
                format!("%{{in}} {}", s)
            }
        } else {
            s
        }
    }
}

/// This format is verbose and will generate strings like "2 years, 5 months, and 3 days ago".
#[derive(Clone, Default)]
pub struct CoarseRoundFormat {
    formats: Vec<ApproximateState>,
    parsed: bool,
}

impl FormatGenerator for CoarseRoundFormat {
    fn set_is_parsed(&mut self) {
        self.parsed = true;
    }

    fn is_parsed(&self) -> bool {
        self.parsed
    }

    fn add(&mut self, states: StateCollection) {
        for state in states.0 {
            match state {
                ApproximateState::Value(..) | ApproximateState::InPast(_) => {
                    self.formats.push(state)
                }
                _ => {}
            }
        }
    }

    fn format(&self) -> String {
        let mut in_past: Option<bool> = None;
        let mut s = String::new();
        let mut last = String::new();
        for (x, format) in self.formats.iter().enumerate() {
            match format {
                ApproximateState::InPast(past) => in_past = Some(*past),
                ApproximateState::Value(relative, time) => {
                    if !last.is_empty() {
                        s += &last;
                        last.truncate(0);
                    }

                    let relative: Words = (*relative).clone().into();
                    let relative = if *time > 1 {
                        relative.plural()
                    } else {
                        relative
                    };

                    if self.formats.len() >= 2 && x < self.formats.len() - 2 {
                        last = format!("{} %{{{}}}, ", time, relative);
                    } else {
                        last = format!("{} %{{{}}} ", time, relative);
                    }
                }
                _ => {}
            }
        }

        if !last.is_empty() {
            if s.is_empty() {
                s += last.trim();
            } else {
                s += &format!("%{{and}} {}", last.trim());
            }
        }

        if let Some(past) = in_past {
            if past {
                s + " %{ago}"
            } else {
                s + " %{from now}"
            }
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coarse_round_format() {
        use super::*;
        use crate::approximate::{ApproximateFilter, Approximator};
        use crate::time_boundary::TimeBoundary;
        use chrono::prelude::*;

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1978, 4, 6).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::ApproximateTime,
                ApproximateFilter::Round(TimeBoundary::Year),
                ApproximateFilter::Round(TimeBoundary::Month),
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            CoarseRoundFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!(
            "45 %{years}, 9 %{months} %{and} 17 %{days} %{ago}",
            states.to_string()
        );

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            CoarseRoundFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("4 %{days} %{ago}", states.to_string());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 4, 3).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::Round(TimeBoundary::Month),
            ],
            CoarseRoundFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("2 %{months} %{from now}", states.to_string());

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::RoundWithBound(TimeBoundary::Month, 2),
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            CoarseRoundFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!(
            "2 %{months} %{and} 26 %{days} %{from now}",
            states.to_string()
        );
    }

    #[test]
    fn test_fancy_duration_format() {
        use super::*;
        use crate::approximate::{ApproximateFilter, Approximator};
        use crate::time_boundary::TimeBoundary;
        use chrono::prelude::*;

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1978, 4, 6).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::ApproximateTime,
                ApproximateFilter::Round(TimeBoundary::Year),
                ApproximateFilter::Round(TimeBoundary::Month),
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            FancyDurationFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("45y9m17d %{ago}", states.to_string());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            FancyDurationFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("4d %{ago}", states.to_string());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 4, 3).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::Round(TimeBoundary::Month),
            ],
            FancyDurationFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("%{in} 2m", states.to_string());

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::Relative,
                ApproximateFilter::RoundWithBound(TimeBoundary::Month, 2),
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            FancyDurationFormat::default(),
        );

        let states = approximator.difference(date, date2);
        assert_eq!("%{in} 2m26d", states.to_string());
    }
}
