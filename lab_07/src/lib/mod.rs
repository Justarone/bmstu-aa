use csv::ReaderBuilder;
use std::cmp::Ord;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};

mod maps;
pub use maps::{BinaryMap, BruteMap, DictEntry, Id, Map, Name, SegMap, SegmentMap};
mod constants;

fn read_data(filename: &str) -> Vec<DictEntry<Id, Name>> {
    let mut file = File::open(filename).unwrap();
    let mut res = Vec::new();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(data.as_bytes());
    for elem in reader.deserialize::<DictEntry<_, _>>() {
        res.push(elem.unwrap());
    }
    res
}

pub fn run_tests() {
    let source = read_data(constants::DATA_PATH);
    let brutemap = BruteMap::from(source.clone());
    let binmap = BinaryMap::from(source.clone());
    let segmap = SegmentMap::from(source.clone(), |k| k % constants::N_OF_SEGMENTS);

    println!("================== BruteMap  ========================");
    measure(brutemap, &source, (&constants::GOOD_KEYS_AMOUNTS, &constants::BAD_KEYS_AMOUNTS));
    println!("=====================================================");
    println!("================== BinaryMap ========================");
    measure(binmap, &source, (&constants::GOOD_KEYS_AMOUNTS, &constants::BAD_KEYS_AMOUNTS));
    println!("=====================================================");
    println!("==================   SegMap  ========================");
    measure(segmap, &source, (&constants::GOOD_KEYS_AMOUNTS, &constants::BAD_KEYS_AMOUNTS));
    println!("=====================================================");
}

fn measure<T: Map<U, V>, U: Ord + Copy + From<usize>, V: Clone>(
    map: T,
    source: &[DictEntry<U, V>],
    (gks, bks): (&[usize], &[usize]),
) {
    let slen = source.len();
    for &ka in gks {
        let gktime = Instant::now();
        for key in (0..ka).map(|i| source[i % slen].key) {
            map.get(&key);
        }
        let gktime = gktime.elapsed().as_nanos();
        println!("GOOD_KEYS:\namount={}\ntime={}\n",
            ka.to_formatted_string(&Locale::en),
            gktime.to_formatted_string(&Locale::en),
        );
    }

    println!("");

    for &ka in bks {
        let bktime = Instant::now();
        for key in (0..ka).map(|i| source[i % slen].key) {
            map.get(&key);
        }
        let bktime = bktime.elapsed().as_nanos();
        println!("BAD_KEYS:\namount={}\ntime={}\n",
            ka.to_formatted_string(&Locale::en), 
            bktime.to_formatted_string(&Locale::en)
        );
    }

}
