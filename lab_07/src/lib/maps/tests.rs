use super::*;

fn check<T: Map<String, String>>(f: fn(Vec<DictEntry<String, String>>) -> T) {
    let map = f(vec![
        DictEntry {
            key: String::from("k1"),
            value: String::from("val1"),
        },
        DictEntry {
            key: String::from("k2"),
            value: String::from("val2"),
        },
        DictEntry {
            key: String::from("k4"),
            value: String::from("val4"),
        },
    ]);

    assert_eq!(map.get(&String::from("k0")), None);
    assert_eq!(map.get(&String::from("k1")), Some(String::from("val1")));
    assert_eq!(map.get(&String::from("k2")), Some(String::from("val2")));
    assert_eq!(map.get(&String::from("k3")), None);
    assert_eq!(map.get(&String::from("k4")), Some(String::from("val4")));
    assert_eq!(map.get(&String::from("k5")), None);
}

#[test]
fn check_brute() {
    check(BruteMap::from);
}

#[test]
fn check_binary() {
    check(BinaryMap::from);
}

#[test]
fn check_segment() {
    check(|v| SegmentMap::from(v, |k| k.clone()));
}
