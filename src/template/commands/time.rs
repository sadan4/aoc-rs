use crate::template::run_multi::run_multi;
use crate::template::timings::Timings;
use crate::template::{all_days, readme_benchmarks, Day};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

fn day_file_exists(day: &Day) -> bool {
    PathBuf::from_str("src")
        .unwrap()
        .join("bin")
        .join(day.to_string())
        .with_extension("rs")
        .exists()
}

pub fn handle(day: Option<Day>, run_all: bool, store: bool) {
    let stored_timings = Timings::read_from_file();

    let days_to_run = day.map_or_else(
        || {
            if run_all {
                all_days().collect()
            } else {
                // when the `--all` flag is not set, filter out days that are fully benched or do not exist.
                all_days()
                    .filter(|day| day_file_exists(day))
                    .filter(|day| !stored_timings.is_day_complete(*day))
                    .collect()
            }
        },
        |day| HashSet::from([day]),
    );

    let timings = run_multi(&days_to_run, true, true).unwrap();

    if store {
        let merged_timings = stored_timings.merge(&timings);
        merged_timings.store_file().unwrap();

        println!();
        match readme_benchmarks::update(merged_timings) {
            Ok(()) => {
                println!("Stored updated benchmarks.");
            }
            Err(_) => {
                eprintln!("Failed to store updated benchmarks.");
            }
        }
    }
}
