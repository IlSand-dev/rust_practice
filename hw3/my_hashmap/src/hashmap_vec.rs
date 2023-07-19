use std::alloc::Layout;
use std::hash::{Hash, Hasher};
use std::{alloc, mem, ptr};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::ptr::{NonNull};

pub struct MyHashMapVec<K: Hash + Eq + Debug, V: Debug> {
    values_ptr: NonNull<Vec<(K, V)>>,
    len: usize,
    capacity: usize,
}

impl<K: Hash + Eq + Debug, V: Debug> MyHashMapVec<K, V> {

    pub fn new() -> Self{
        MyHashMapVec{
            values_ptr: NonNull::dangling(),
            len: 0,
            capacity: 0
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.len == self.capacity { self.grow() };
        let index = self.get_index(&key);
        let mut values: Vec<(K, V)> = unsafe { ptr::read(self.values_ptr.as_ptr().add(index)) };
        values.reserve(1);
        println!("{} {} {}", values.len(), values.capacity(), self.capacity);
        'a: {
            for (i, (k, _)) in values.iter().enumerate() {
                if k.eq(&key) {
                    values[i] = (key, value);
                    break 'a;
                }
            }
            println!("values: {:?}; key: {:?}; value:{:?}", values.capacity(), key, value);
            values.push((key, value));
        }
        println!("values: {:?}", values);
        unsafe { ptr::write(self.values_ptr.as_ptr().add(index), values) };
        self.len += 1;
    }

    pub fn get(&self, key: K) -> Option<V>{
        let index = self.get_index(&key);
        let values = unsafe { ptr::read(self.values_ptr.as_ptr().add(index)) };

        for (k, v) in values {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: K) {
        let index = self.get_index(&key);
        let mut values = unsafe { ptr::read(self.values_ptr.as_ptr().add(index)) };
        for (i, (k, _)) in values.iter().enumerate() {
            if k.eq(&key) {
                values.remove(i);
                break;
            }
        }
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & self.capacity
    }
    fn grow(&mut self) {
        assert_ne!(mem::size_of::<K>(), 0, "Capacity overflow");
        assert_ne!(mem::size_of::<V>(), 0, "Capacity overflow");

        let (new_capacity, new_layout_values) = if self.capacity == 0 {
            (1, Layout::array::<Vec<(K, V)>>(1).unwrap())
        } else {
            let new_capacity = 2 * self.capacity;
            (new_capacity, Layout::array::<Vec<(K, V)>>(new_capacity).unwrap())
        };
        assert!(new_layout_values.size() <= isize::MAX as usize, "Allocation too large");

        if self.capacity == 0 {
            let new_values_ptr = unsafe {
                alloc::alloc(new_layout_values)
            };
            self.values_ptr = match NonNull::new(new_values_ptr as *mut Vec<(K, V)>) {
                Some(v) => v,
                None => alloc::handle_alloc_error(new_layout_values)
            };
            self.capacity = new_capacity;
            unsafe {ptr::write(self.values_ptr.as_ptr().add(0),Vec::with_capacity(self.capacity))};
        } else {
            let old_layout = Layout::array::<Vec<(K, V)>>(self.capacity).unwrap();
            let old_ptr = self.values_ptr.as_ptr();
            let new_ptr = unsafe { alloc::alloc(new_layout_values) };
            self.values_ptr = match NonNull::new(new_ptr as *mut Vec<(K, V)>) {
                Some(v) => v,
                None => alloc::handle_alloc_error(new_layout_values)
            };
            let old_capacity = self.capacity;
            self.capacity = new_capacity;
            self.reinsert_values(old_ptr, old_capacity);
            unsafe { alloc::dealloc(old_ptr as *mut u8, old_layout) }
        }
    }

    fn reinsert_values(&mut self, old_ptr: *mut Vec<(K, V)>, old_capacity: usize) {
        for i in 0..old_capacity {
            let old_values = unsafe { ptr::read(old_ptr.add(i)) };
            unsafe {ptr::write(old_ptr.add(i),Vec::with_capacity(self.capacity))};
            for (key, value) in old_values {
                self.insert(key, value);
            }
        }
    }
}


impl<K: Hash + Eq + Debug, V: Debug> Drop for MyHashMapVec<K, V> {
    fn drop(&mut self) {
        for i in 0..self.capacity {
            let values = unsafe { ptr::read(self.values_ptr.as_ptr().add(i)) };
            for (_, _) in values{};
        }
        if self.capacity != 0 {
            unsafe {
                alloc::dealloc(self.values_ptr.as_ptr() as *mut u8,
                               Layout::array::<Vec<(K, V)>>(self.capacity).unwrap())
            }
        }
    }
}