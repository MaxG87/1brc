use std::collections::BTreeSet;
use std::env;
use std::io::{self, BufWriter, Write};

use rand::{
    distributions::{Alphanumeric, DistIter, Distribution, Slice, Uniform},
    rngs::StdRng,
    SeedableRng,
};

const MIN_VALUE: i16 = -999; // inclusive
const MAX_VALUE: i16 = 999; // inclusive
const MIN_CITY_NAME_LEN: usize = 1;
const MAX_CITY_NAME_LEN: usize = 32; // inclusive

fn get_city_name(
    city_name_len_rng: &mut DistIter<Uniform<usize>, StdRng, usize>,
    city_name_char_rng: &mut DistIter<Alphanumeric, StdRng, u8>,
) -> String {
    // Sampling printable UTF8 characters would be overly complex. Therefore,
    // only ASCII characters are sampled.
    let city_name_len = city_name_len_rng.next().unwrap();
    let city_name_bytes = city_name_char_rng.take(city_name_len).collect::<Vec<_>>();
    let city_name = String::from_utf8(city_name_bytes)
        .expect("Sampling ASCII characters must be valid UTF8");
    city_name.to_string()
}

fn get_cities(
    nof_cities: u32,
    city_name_len_rng: &mut DistIter<Uniform<usize>, StdRng, usize>,
    city_name_char_rng: &mut DistIter<Alphanumeric, StdRng, u8>,
) -> Vec<String> {
    let mut cities = BTreeSet::new();
    for _ in 0..nof_cities {
        loop {
            let city_name = get_city_name(city_name_len_rng, city_name_char_rng);
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
    let maybe_seed = args
        .get(3)
        .map(|s| s.parse().expect("Seed must be a positive integer!"));
    let mut seed_rng = match maybe_seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let mut city_name_len_rng =
        Uniform::new_inclusive(MIN_CITY_NAME_LEN, MAX_CITY_NAME_LEN)
            .sample_iter(StdRng::from_rng(&mut seed_rng).unwrap());
    let mut city_name_char_rng =
        Alphanumeric.sample_iter(StdRng::from_rng(&mut seed_rng).unwrap());
    let cities = get_cities(
        max_nof_cities,
        &mut city_name_len_rng,
        &mut city_name_char_rng,
    );

    let mut value_rng =
        Uniform::new_inclusive(f32::from(MIN_VALUE), f32::from(MAX_VALUE))
            .sample_iter(StdRng::from_rng(&mut seed_rng).unwrap());
    let mut city_rng = Slice::new(&cities)
        .expect("No cities provided!")
        .sample_iter(StdRng::from_rng(&mut seed_rng).unwrap());

    let mut lock = io::stdout().lock();
    let mut writer = BufWriter::with_capacity(8192, &mut lock);
    for _ in 0..nof_rows {
        let city = city_rng.next().unwrap();
        let value = value_rng.next().unwrap();
        let value = f64::from(value) / 10.0;
        match writeln!(writer, "{city};{value:.1}") {
            Ok(()) => (),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::BrokenPipe => (),
                    _ => eprintln!("Error writing to STDOUT: {e}"),
                }
                break;
            }
        }
    }
    writer.flush().expect("Error flushing buffer!");
}
