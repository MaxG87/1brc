use std::collections::HashSet;
use std::env;
use std::io::{self, Write};

#[cfg(feature = "random")]
use rand::{
    distributions::{Alphanumeric, Distribution, Slice, Uniform},
    Rng,
};

const MIN_VALUE: i16 = -999; // inclusive
const MAX_VALUE: i16 = 999; // inclusive
const MIN_CITY_NAME_LEN: usize = 1;
const MAX_CITY_NAME_LEN: usize = 32; // inclusive

fn get_city_name() -> String {
    // Sampling printable UTF8 characters would be overly complex. Therefore,
    // only ASCII characters are sampled.
    let city_name_len =
        rand::thread_rng().gen_range(MIN_CITY_NAME_LEN..=MAX_CITY_NAME_LEN);
    let char_rng = Alphanumeric.sample_iter(rand::thread_rng());
    let city_name_bytes = char_rng.take(city_name_len).collect::<Vec<_>>();
    let city_name = String::from_utf8(city_name_bytes)
        .expect("Sampling ASCII characters must be valid UTF8");
    city_name.to_string()
}

fn get_cities(nof_cities: u32) -> Vec<String> {
    let mut cities = HashSet::new();
    for _ in 0..nof_cities {
        loop {
            let city_name = get_city_name();
            if city_name.contains(';') {
                continue;
            }
            if cities.insert(city_name) {
                break;
            }
        }
    }
    cities.into_iter().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_nof_cities: u32 = args
        .get(1)
        .expect("Maximum number of cities not provided!")
        .parse()
        .expect("Maximum number of cities must be a positive integer!");
    let nof_rows: u32 = args
        .get(2)
        .expect("Number of rows to generate not provided!")
        .parse()
        .expect("Number of rows to generate must be a positive integer!");

    let cities = get_cities(max_nof_cities);
    let mut value_rng =
        Uniform::new_inclusive(f32::from(MIN_VALUE), f32::from(MAX_VALUE))
            .sample_iter(rand::thread_rng());
    let mut city_rng = Slice::new(&cities)
        .expect("No cities provided!")
        .sample_iter(rand::thread_rng());
    let mut lock = io::stdout().lock();
    for _ in 0..nof_rows {
        let city = city_rng.next().unwrap();
        let value = value_rng.next().unwrap();
        let value = f64::from(value) / 10.0;
        writeln!(lock, "{city};{value:.1}").unwrap();
    }
}
