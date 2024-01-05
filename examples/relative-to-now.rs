use anyhow::Result;
use speakable_time::prelude::*;
use std::io::Read;

fn main() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let approx = approximator!(
        CoarseRoundFormat::default(),
        ApproximateFilter::TopRounds(4),
        ApproximateFilter::Relative
    );

    for line in buf.split('\n') {
        let dt = chrono::DateTime::parse_from_rfc2822(line.trim())?.into();
        println!("{}", from_now!(dt, approx).unwrap());
    }

    Ok(())
}
