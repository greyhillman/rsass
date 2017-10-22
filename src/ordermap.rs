use std::slice::Iter;
use std::vec::IntoIter;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderMap<K, V>(Vec<(K, V)>);

impl<K: Clone + PartialEq, V> OrderMap<K, V> {
    pub fn new() -> Self {
        OrderMap(Vec::new())
    }
    pub fn insert(&mut self, key: K, value: V) {
        for &mut (ref k, ref mut v) in self.0.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }
        self.0.push((key, value));
    }
    pub fn iter(&self) -> Iter<(K, V)> {
        self.0.iter()
    }
    // TODO Should return a specialized iterator!
    pub fn keys(&self) -> Vec<K> {
        self.0.iter().map(|&(ref k, ref _v)| k).cloned().collect()
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        for &(ref k, ref v) in self.0.iter() {
            if k == key {
                return Some(v)
            }
        }
        None
    }
    pub fn remove(&mut self, key: &K) {
        self.0.retain(|&(ref k, ref _v)| k != key);
    }
    pub fn contains_key(&self, key: &K) -> bool {
        for &(ref k, ref _v) in self.0.iter() {
            if k == key {
                return true
            }
        }
        false
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K, V> IntoIterator for OrderMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;
    fn into_iter(self) -> IntoIter<(K, V)> {
        self.0.into_iter()
    }
}

impl<K, V> FromIterator<(K, V)> for OrderMap<K, V> {
    fn from_iter<T>(i: T) -> Self
        where T: IntoIterator<Item = (K, V)>
    {
        OrderMap(Vec::from_iter(i))
    }
}
