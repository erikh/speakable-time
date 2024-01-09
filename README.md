# Human-friendly time heavily abstracted

Speakable time in this case, are time periods or intervals that are used informally, times like "10 years ago" or "next week". This library aims to deconstruct times in a way that they can be treated like a grammar for generating human-friendly times.

To accomplish this, we seed an Approximator with a list of selectors called ApproximateFilters, and a FormatGenerator, of which there are currently two styles: CoarseRoundFormat and FancyDurationFormat. The approximator computes durations into tokenized parts (called ApproximateState) which are picked out by the ApproximateFilters, which are then fed to the formatter of choice to generate a syntax. Then, a [Translator] is used to convert that final syntax into something you can read. It is fully localized and flexible.

There a few macros to make this easier. If the `translation` feature is enabled, it will allow you to provide translation maps as YAML files that live in your build directory. It will then select the right mapping to use for a given locale automatically on boot.

There are plenty of docs for writing your own formatter, too. Just see FormatGenerator.

## Example

```rust
use speakable_time::prelude::*;

let approx = approximator!(
    // Use a format that lays heavily on the english
    CoarseRoundFormat::default(),
    // Round to the nearest day
    ApproximateFilter::Round(TimeBoundary::Day),
    // Indicate whether or not the compared time is in the future or past
    ApproximateFilter::Relative
);

let dt = chrono::Local::now() - chrono::Duration::days(2);

// from_now! compares the dt against the current time and applies the approximator, and runs
// it through the default translation.
assert_eq!("2 days ago", from_now!(dt, approx).unwrap());

// Here we'll round to the nearest year and day, but not indicate future or past. Watch how
// the output changes!
let approx = approximator!(
    CoarseRoundFormat::default(),
    ApproximateFilter::Round(TimeBoundary::Year),
    ApproximateFilter::Round(TimeBoundary::Day)
);

let dt2 = chrono::Local::now() + chrono::Duration::days(45 * 365 + 2); // account for leap
                                                                   // years
assert_eq!("45 years and 4 days", time_diff!(dt2, dt, approx).unwrap());

let approx = approximator!(
    // Here we use an abbreviated format that is also used by the `fancy-duration` crate.
    FancyDurationFormat::default(),
    // We're going to round to the two largest values
    ApproximateFilter::TopRounds(2),
    // Show past or present
    ApproximateFilter::Relative
);

assert_eq!("in 45y4d", time_diff!(dt2, dt, approx).unwrap());

// Here's the same approximation applied to a date that is just 2 days from now
assert_eq!(
    "in 2d",
    time_diff!(
        chrono::Local::now() + chrono::Duration::days(2) + chrono::Duration::seconds(1),
        chrono::Local::now(),
        approx
    ).unwrap()
);
```

## Examples

There is an easily tweakable example in [relative-to-now](examples/relative-to-now.rs) that just accepts RFC 2822 formatted dates and converts them based on how far away they are from now. The [generate-random-times](examples/generate-random-times.rs) can be used to feed it so you can see a lot of conversions happen at once.

## Tests

```
cargo test --all-features
```

## More to do

Formatting can be a lot more intricate and there could be more options. I also want to bake in some translations too.

## Author

Erik Hollensbe <git@hollensbe.org>

## License

MIT
