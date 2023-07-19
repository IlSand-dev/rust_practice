use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub struct MyVecHashMap<K: Hash + PartialEq, V> {
    values: Vec<Vec<(K, V)>>,
}

impl<K: Hash + PartialEq, V: Debug> MyVecHashMap<K, V> {
    pub fn new() -> Self {
        MyVecHashMap {
            values: Vec::new()
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let old_cap = self.values.capacity();
        self.values.reserve(1);
        if old_cap != self.values.capacity() {
            self.reinsert_values();
        }
        let index = self.get_index(&key);
        match self.values.get_mut(index) {
            Some(vals) => {
                'a:{
                    for (i, (k, _)) in vals.iter().enumerate(){
                        if *k == key{
                            vals[i] = (key, value);
                            break 'a;
                        }
                    }

                }
            }
            None => self.values[index].push((key, value))
        };
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & self.values.capacity()
    }

    fn reinsert_values(&mut self) {
        for vals in self.values {
            for (key, value) in vals {
                self.insert(key, value);
            }
        }
    }
}