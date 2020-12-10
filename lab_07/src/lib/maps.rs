use serde_derive::Deserialize;
use std::cmp::{Ord, Ordering};

pub type Id = usize;
pub type Name = String;

#[derive(Clone, Debug, Deserialize)]
pub struct DictEntry<K: Ord, V: Clone> {
    pub key: K,
    pub value: V,
}

pub trait Map<K: Ord, V: Clone> {
    fn get(&self, key: &K) -> Option<V>;
}

#[derive(Debug)]
pub struct BruteMap<K: Ord, V: Clone> {
    data: Vec<DictEntry<K, V>>,
}

impl<K: Ord, V: Clone> From<Vec<DictEntry<K, V>>> for BruteMap<K, V> {
    fn from(source: Vec<DictEntry<K, V>>) -> BruteMap<K, V> {
        BruteMap { data: source }
    }
}

impl<K: Ord, V: Clone> Map<K, V> for BruteMap<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        self.data
            .iter()
            .find(|&e| e.key == *key)
            .map(|entry| entry.value.clone())
    }
}

pub struct BinaryMap<K: Ord, V: Clone> {
    data: Vec<DictEntry<K, V>>,
}

impl<K: Ord, V: Clone> From<Vec<DictEntry<K, V>>> for BinaryMap<K, V> {
    fn from(mut source: Vec<DictEntry<K, V>>) -> BinaryMap<K, V> {
        source.sort_by(|a, b| a.key.cmp(&b.key));
        BinaryMap { data: source }
    }
}

fn bsearch<T: Ord, U: Clone>(arr: &[DictEntry<T, U>], key: &T) -> Option<U> {
    let (mut left, mut right) = (0 as i64, arr.len() as i64 - 1);
    while left <= right {
        let m = ((left + right) / 2) as usize;
        match arr[m].key.cmp(&key) {
            Ordering::Less => left = m as i64 + 1,
            Ordering::Greater => right = m as i64 - 1,
            Ordering::Equal => return Some(arr[m].value.clone()),
        }
    }
    None
}

impl<K: Ord, V: Clone> Map<K, V> for BinaryMap<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        bsearch(&self.data, key)
    }
}

pub struct SegmentMap<G: Ord, K: Ord + Clone, V: Clone> {
    data: Vec<DictEntry<G, Vec<DictEntry<K, V>>>>,
    f: fn(&K) -> G,
}

pub trait SegMap<G: Ord, K: Ord, V> {
    fn get(&self, key: &K) -> Option<V>;
}

impl<G: Ord, K: Ord + Clone, V: Clone> SegmentMap<G, K, V>
where
    G: Ord,
    K: Ord,
{
    fn put(&mut self, pair: DictEntry<K, V>) {
        let seg_key = (self.f)(&pair.key);
        let inner = match self.data.iter().position(|e| e.key == seg_key) {
            Some(p) => &mut self.data[p].value,
            None => {
                self.data.push(DictEntry {
                    key: seg_key,
                    value: Vec::new(),
                });
                let index = self.data.len() - 1;
                &mut self.data[index].value
            }
        };
        inner.push(pair);
    }

    pub fn from(source: Vec<DictEntry<K, V>>, f: fn(&K) -> G) -> SegmentMap<G, K, V> {
        let mut m = SegmentMap {
            f,
            data: Vec::new(),
        };
        for e in source {
            m.put(e);
        }
        m.data.sort_by(|a, b| b.value.len().cmp(&a.value.len()));
        m
    }
}

impl<G: Ord, K: Ord + Clone, V: Clone> SegMap<G, K, V> for SegmentMap<G, K, V> {
    fn get(&self, key: &K) -> Option<V> {
        let seg_key = (self.f)(&key);
        let inner = match self.data.iter().position(|e| e.key == seg_key) {
            Some(pos) => &self.data[pos].value,
            None => return None,
        };
        bsearch(inner, &key)
    }
}

impl<K: Ord + Clone, V: Clone> Map<K, V> for SegmentMap<K, K, V> {
    fn get(&self, key: &K) -> Option<V> {
        <SegmentMap<K, K, V> as SegMap<K, K, V>>::get(&self, key)
    }
}

#[cfg(test)]
mod tests;
