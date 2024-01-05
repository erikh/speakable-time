pub mod formats;

pub use self::formats::{CoarseRoundFormat, FancyDurationFormat};
use super::StateCollection;

/// Implement this trait to generate a format. This consumes
/// [ApproximateState](crate::approximate::ApproximateState) and produces a string format,
/// ultimately. It sets a parsed state when the format is considered "baked", making it easy for
/// formatters to repeat the same value without recalculating.
pub trait FormatGenerator {
    fn set_is_parsed(&mut self);
    fn is_parsed(&self) -> bool;
    fn add(&mut self, state: StateCollection);
    fn format(&self) -> String;
}

/// Used for testing mostly, always yields an empty string, is always parsed.
#[derive(Clone)]
pub struct EmptyFormatGenerator;

impl FormatGenerator for EmptyFormatGenerator {
    fn set_is_parsed(&mut self) {}
    fn is_parsed(&self) -> bool {
        true
    }
    fn add(&mut self, _: StateCollection) {}
    fn format(&self) -> String {
        String::new()
    }
}
