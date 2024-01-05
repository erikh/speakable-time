/// This is the entrypoint to the formatting subsystem that convert [ApproximateState] values to
/// strings which can then be run through the [Translator](crate::translator::Translator).
pub mod format_generator;

pub use self::format_generator::formats::{CoarseRoundFormat, FancyDurationFormat};

use self::format_generator::FormatGenerator;
use super::enums::{Month, Weekday, MONTH, WEEK, YEAR};
use crate::time_boundary::TimeBoundary;
use chrono::prelude::*;
use chrono::Duration;

/// Filter types for the approximator. These help the approximator make decisions about what is
/// relevant to your duration.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApproximateFilter {
    /// Get the top (value) boundaries that round cleanly and are non-zero.
    TopRounds(usize),
    /// Get the top (value) boundaries that round cleanly and are non-zero. A boundary is provided
    /// for the highest level that can be considered a valid boundary.
    TopRoundsMaxRelative(usize, TimeBoundary),
    /// With a provided boundary, round up to this many multiples of that boundary before
    /// discarding the attempt to round.
    RoundWithBound(TimeBoundary, i64),
    /// With a provided boundary, provide the value if the boundary modulos cleanly into the
    /// duration and has a non-zero remainder.
    Round(TimeBoundary),
    /// If the duration is within one week, provide the day name of the compared time.
    DayNameWithinWeek,
    /// If the duration is within one year, provide the month name of the compared time.
    MonthNameWithinYear,
    /// Provide noon and midnight for those situations.
    HourShorthand,
    /// See [ApproximateTime]. One of Morning, Afternoon, Evening, or Night
    /// depending on time of day.
    ApproximateTime,
    /// Returns InPast(bool) where the value is true if the compared time is previous to the original time.
    Relative,
}

/// This is an approximate time of day, such as Morning, Afternoon, Evening, or Night.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApproximateTime {
    Morning,
    Afternoon,
    Evening,
    Night,
}

/// These are the states returned by the [Approximator]. Usually you will get
/// several of them, and they are dependent based on what
/// [ApproximateFilter]s you configured. Each one holds different
/// information which can be used to format your time.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApproximateState {
    /// This contains a boundary and the amount of time the duration consumed of that boundary.
    /// This is used in rounding filters mostly, and several are often returned with different
    /// boundaries. The boundaries are returned in order of precedence.
    Value(TimeBoundary, i64),
    /// This is true if the compared time is previous to the original time.
    InPast(bool),
    /// This produces an [ApproximateTime] which indicated time of day.
    ApproximateTime(ApproximateTime),
    /// This produces a [Weekday] which indicates the day of the week.
    DayName(Weekday),
    /// This produces a [Month] which indicates the name of the month.
    MonthName(Month),
    /// This produces "noon" or "midnight" if the time is (24h) 12:00 or 00:00 respectively.
    HourShorthand(String),
    /// This is the date of the compared value separated out
    WithDate(NaiveDate),
    /// This is the time of the compared value separated out
    WithTime(NaiveTime),
}

/// This is a container for [ApproximateState] values. These are returned in this shell so that
/// they can be checked when generating a grammar.
#[derive(Debug, Clone, Default)]
pub struct StateCollection(Vec<ApproximateState>);

impl StateCollection {
    /// Does this collection contain the state?
    #[inline]
    pub fn contains(&self, key: &ApproximateState) -> bool {
        self.0.contains(key)
    }

    /// Add a new state to the collection.
    #[inline]
    pub fn push(&mut self, key: ApproximateState) {
        self.0.push(key)
    }
}

/// The state formatter drives the [FormatGenerator]
/// against the [StateCollection].
#[derive(Debug, Clone)]
pub struct StateFormatter<T>
where
    T: FormatGenerator + Clone,
{
    states: StateCollection,
    obj: T,
}

impl<T> StateFormatter<T>
where
    T: FormatGenerator + Clone,
{
    /// Does this collection contain the state?
    #[inline]
    pub fn contains(&self, key: &ApproximateState) -> bool {
        self.states.contains(key)
    }

    /// Add the states to the [FormatGenerator] and run
    /// its internal parser.
    #[inline]
    pub fn parse(&mut self) {
        self.obj.add(self.states.clone());
        self.obj.set_is_parsed();
    }

    /// If the collection is still unparsed, parse it; then format the string.
    #[inline]
    pub fn format(&mut self) -> String {
        if !self.obj.is_parsed() {
            self.parse();
        }
        self.obj.format()
    }

    /// This is a slightly faster version of format for situations where you will re-use the
    /// format.
    #[inline]
    pub fn format_parsed(&self) -> String {
        if self.obj.is_parsed() {
            self.obj.format()
        } else {
            self.clone().format()
        }
    }

    /// Return the [StateCollection] being held by this formatter.
    #[inline]
    pub fn states(&self) -> StateCollection {
        self.states.clone()
    }
}

impl<T> std::fmt::Display for StateFormatter<T>
where
    T: FormatGenerator + Clone,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format_parsed())
    }
}

/// The [Approximator] accepts a list of [ApproximateFilter] and a
/// [FormatGenerator]. The engine will run against the
/// different filter states and generate a list of tokens, which can then be trivially fed to the
/// formatter. Note, the string result still needs to be run through a
/// [Translator](crate::translator::Translator) to be turned into something intended for humans.
pub struct Approximator<T>
where
    T: FormatGenerator + Clone,
{
    filter: Vec<ApproximateFilter>,
    obj: T,
}

impl<T> Approximator<T>
where
    T: FormatGenerator + Clone,
{
    /// Construct a new object with the list of filters and the formatter.
    pub fn new(filter: Vec<ApproximateFilter>, obj: T) -> Self {
        Self { filter, obj }
    }

    /// Compute the difference of the current time and the provided time.
    #[inline]
    pub fn from_now(&self, dt: DateTime<Local>) -> StateFormatter<T> {
        self.difference(dt, Local::now())
    }

    /// Compute the difference of two times. The first time is considered the "original", and the
    /// second the "compared" values when generating results in [ApproximateState] results.
    pub fn difference(&self, dt: DateTime<Local>, against: DateTime<Local>) -> StateFormatter<T> {
        let mut state = StateCollection::default();

        state.push(ApproximateState::WithDate(dt.date_naive()));
        state.push(ApproximateState::WithTime(dt.time()));

        let mut duration = crate::absolute_duration(dt, against);

        'item: for item in &self.filter {
            match item {
                ApproximateFilter::TopRounds(count) => {
                    let mut added = 0;
                    for relative in TimeBoundary::all() {
                        if added >= *count {
                            continue 'item;
                        }

                        let new_state = match_relative(&mut duration, relative, None);
                        if let Some(new_state) = new_state {
                            added += 1;
                            state.push(new_state);
                        }
                    }
                }
                ApproximateFilter::TopRoundsMaxRelative(count, relative) => {
                    let mut added = 0;
                    for cur in &TimeBoundary::all() {
                        if added >= *count {
                            continue 'item;
                        }

                        if relative < cur {
                            continue;
                        }

                        let new_state = match_relative(&mut duration, cur.clone(), None);
                        if let Some(new_state) = new_state {
                            added += 1;
                            state.push(new_state);
                        }
                    }
                }
                ApproximateFilter::Relative => {
                    // do not use duration here, it is absolute by now
                    state.push(ApproximateState::InPast(
                        against - dt > Duration::seconds(0),
                    ));
                }
                ApproximateFilter::ApproximateTime => {
                    let hour = dt.hour();

                    let approx_time = if hour < 6 {
                        ApproximateTime::Night
                    } else if hour < 12 {
                        ApproximateTime::Morning
                    } else if hour < 18 {
                        ApproximateTime::Afternoon
                    } else {
                        ApproximateTime::Evening
                    };

                    state.push(ApproximateState::ApproximateTime(approx_time));
                }
                ApproximateFilter::HourShorthand => {
                    let shorthand = if dt.hour() == 0 {
                        Some("midnight")
                    } else if dt.hour() == 12 {
                        Some("noon")
                    } else {
                        None
                    };

                    if let Some(hour) = shorthand {
                        state.push(ApproximateState::HourShorthand(hour.to_string()));
                    }
                }
                ApproximateFilter::MonthNameWithinYear => {
                    if duration.num_seconds() < YEAR {
                        let name: Month = dt.month0().into();
                        state.push(ApproximateState::MonthName(name));
                    }
                }
                ApproximateFilter::DayNameWithinWeek => {
                    if duration.num_seconds() < WEEK {
                        let name: Weekday = dt.weekday().into();
                        state.push(ApproximateState::DayName(name));
                    }
                }
                ApproximateFilter::Round(relative) => {
                    let new_state = match_relative(&mut duration, relative.clone(), None);
                    if let Some(new_state) = new_state {
                        state.push(new_state)
                    }
                }
                ApproximateFilter::RoundWithBound(relative, upto) => {
                    let new_state = match_relative(&mut duration, relative.clone(), Some(*upto));
                    if let Some(new_state) = new_state {
                        state.push(new_state)
                    }
                }
            }
        }

        StateFormatter {
            states: state,
            obj: self.obj.clone(),
        }
    }
}

/// convenience function to convert time ranges based on their boundaries.
fn match_relative(
    duration: &mut Duration,
    relative: TimeBoundary,
    upto: Option<i64>,
) -> Option<ApproximateState> {
    let result: (&dyn Fn(&Duration) -> i64, &dyn Fn(i64) -> Duration) = match relative {
        TimeBoundary::Year => (&|dur: &Duration| dur.num_seconds() / YEAR, &|count| {
            Duration::seconds(count * YEAR)
        }),
        TimeBoundary::Month => (&|dur: &Duration| dur.num_seconds() / MONTH, &|count| {
            Duration::seconds(count * MONTH)
        }),
        TimeBoundary::Week => (&Duration::num_weeks, &Duration::weeks),
        TimeBoundary::Day => (&Duration::num_days, &Duration::days),
        TimeBoundary::Hour => (&Duration::num_hours, &Duration::hours),
        TimeBoundary::Minute => (&Duration::num_minutes, &Duration::minutes),
        TimeBoundary::Second => (&Duration::num_seconds, &Duration::seconds),
    };

    let count = result.0(duration);

    let result = if count > 0 {
        upto.map_or_else(
            || Some((count, result.1(count))),
            |u| {
                if count <= u {
                    Some((count, result.1(count)))
                } else {
                    None
                }
            },
        )
    } else {
        None
    };

    if let Some(result) = result {
        *duration = *duration - result.1;
        Some(ApproximateState::Value(relative, result.0))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_approximation() {
        use super::format_generator::EmptyFormatGenerator;
        use super::*;
        use crate::time_boundary::TimeBoundary;

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
            EmptyFormatGenerator,
        );

        let states = approximator.difference(date, date2);

        assert!(states.contains(&ApproximateState::InPast(true)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Year, 45)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Month, 9)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Day, 17)));
        assert!(states.contains(&ApproximateState::ApproximateTime(ApproximateTime::Morning)));
        assert!(states.contains(&ApproximateState::WithTime(
            NaiveTime::from_hms_opt(6, 0, 0).unwrap()
        )));
        assert!(states.contains(&ApproximateState::WithDate(
            NaiveDate::from_ymd_opt(1978, 4, 6).unwrap()
        )));

        let approximator =
            Approximator::new(vec![ApproximateFilter::TopRounds(4)], EmptyFormatGenerator);

        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Year, 45)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Month, 9)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Week, 2)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Day, 3)));

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
            vec![ApproximateFilter::DayNameWithinWeek],
            EmptyFormatGenerator,
        );

        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::DayName(Weekday::Wednesday)));

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
            vec![ApproximateFilter::MonthNameWithinYear],
            EmptyFormatGenerator,
        );

        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::MonthName(Month::April)));

        let approximator = Approximator::new(
            vec![
                ApproximateFilter::RoundWithBound(TimeBoundary::Month, 2),
                ApproximateFilter::Round(TimeBoundary::Day),
            ],
            EmptyFormatGenerator,
        );

        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Month, 2)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Day, 26)));

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(6, 12, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();

        let date2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            NaiveTime::from_hms_opt(7, 15, 0).unwrap(),
        )
        .and_local_timezone(chrono::Local)
        .unwrap();
        let approximator = Approximator::new(
            vec![ApproximateFilter::TopRoundsMaxRelative(
                3,
                TimeBoundary::Hour,
            )],
            EmptyFormatGenerator,
        );
        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Hour, 1)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Minute, 3)));

        let approximator = Approximator::new(
            vec![ApproximateFilter::TopRoundsMaxRelative(
                3,
                TimeBoundary::Year,
            )],
            EmptyFormatGenerator,
        );
        let states = approximator.difference(date, date2);
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Hour, 1)));
        assert!(states.contains(&ApproximateState::Value(TimeBoundary::Minute, 3)));

        let approximator = Approximator::new(
            vec![ApproximateFilter::TopRoundsMaxRelative(
                3,
                TimeBoundary::Second,
            )],
            EmptyFormatGenerator,
        );
        let states = approximator.difference(date, date2);
        assert!(!states.contains(&ApproximateState::Value(TimeBoundary::Hour, 1)));
        assert!(!states.contains(&ApproximateState::Value(TimeBoundary::Minute, 3)));
    }
}
