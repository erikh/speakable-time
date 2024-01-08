use anyhow::Result;
use chrono::{Duration, Local};

fn main() -> Result<()> {
    let count: usize = std::env::args()
        .nth(1)
        .expect("Expected a count of dates to generate")
        .parse()?;

    for _ in 0..count {
        println!(
            "{}",
            (Local::now() + Duration::seconds(rand::random::<i64>() % (2 * 365 * 24 * 60 * 60)))
                .to_rfc2822()
        );
    }

    Ok(())
}
