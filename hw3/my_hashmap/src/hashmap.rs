use std::alloc::Layout;
use std::hash::{Hash, Hasher};
use std::{alloc, mem, ptr};
use std::cell::{Ref, RefCell};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::{Rc, Weak};

struct Value<K, V> {
    key: K,
    data: V,
    next: Option<Box<Value<K, V>>>,
}

impl<K, V> Value<K, V> {
    fn get_by_index(&self, index: usize) -> Option<Value<K, V>>{
        for _ in 0..index{
        }
    }
}

pub struct MyHashMap<K: Hash + PartialEq, V> {
    values_ptr: NonNull<Value<K, V>>,
    len: usize,
    capacity: usize,
}

impl<K: Hash + PartialEq, V> MyHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            values_ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        if self.len == self.capacity { self.grow() }
        let index: usize = self.get_index(&key);
        let mut v = Rc::new(RefCell::new(unsafe { ptr::read(self.values_ptr.as_ptr().add(index)) }));
        'l: loop {
            // println!("{:?}", v);
            let v = match v.borrow().next.clone() {
                Some(new_v) => new_v,
                None => {
                    v.borrow_mut().next = Some(Rc::new(
                        RefCell::new(
                            Value {
                                key,
                                data: value,
                                previous: Some(Rc::downgrade(&v)),
                                next: None,
                            })));
                    break 'l;
                }
            };
            if key == v.borrow().key {
                v.borrow_mut().data = value;
                break;
            }
        }
        self.len += 1;
    }

    pub fn get(&self, key: K) -> Option<RefCell<V>> {
        let index = self.get_index(&key);
        let mut v = Rc::new(RefCell::new(unsafe { ptr::read(self.values_ptr.as_ptr().add(index)) }));
        loop {
            match v.borrow().next.clone() {
                Some(child) => child,
                None => return None
            };
            if v.borrow().key == key {
                return Some(v.borrow().data);
            }
        }
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & self.capacity
    }

    fn reinsert_values(&mut self, old_ptr: *mut Value<K, V>, old_capacity: usize) {
        for i in 0..old_capacity {
            let v = Rc::new(RefCell::new(unsafe { ptr::read(old_ptr.add(i)) }));
            loop {
                let v = match v.borrow().next.clone() {
                    Some(new_v) => {
                        new_v
                    }
                    None => break
                };
                self.insert(v.borrow().deref().key, v.borrow().data);
            }
        }
    }


    fn grow(&mut self) {
        assert_ne!(mem::size_of::<V>(), 0, "Capacity overflow");

        let (new_capacity, new_layout_values) = if self.capacity == 0 {
            (1, Layout::array::<Value<K, V>>(1).unwrap())
        } else {
            let new_capacity = 2 * self.capacity;
            (new_capacity, Layout::array::<Value<K, V>>(new_capacity).unwrap())
        };

        assert!(new_layout_values.size() <= isize::MAX as usize, "Allocation too large");

        if self.capacity == 0 {
            let new_values_ptr = unsafe {
                alloc::alloc(new_layout_values)
            };
            self.values_ptr = match NonNull::new(new_values_ptr as *mut Value<K, V>) {
                Some(v) => v,
                None => alloc::handle_alloc_error(new_layout_values)
            };
            self.capacity = new_capacity;
        } else {
            let old_layout = Layout::array::<Value<K, V>>(self.capacity).unwrap();
            let old_ptr = self.values_ptr.as_ptr();
            let new_ptr = unsafe { alloc::alloc(new_layout_values) };
            self.values_ptr = match NonNull::new(new_ptr as *mut Value<K, V>) {
                Some(v) => v,
                None => alloc::handle_alloc_error(new_layout_values)
            };
            let old_capacity = self.capacity;
            self.capacity = new_capacity;
            self.reinsert_values(old_ptr, old_capacity);
            unsafe { alloc::dealloc(old_ptr as *mut u8, old_layout) }
        }
    }
}


impl<K: Hash + PartialEq, V> Drop for MyHashMap<K, V> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            for i in 0..self.capacity {
                let _ = unsafe { ptr::read(self.values_ptr.as_ptr().add(i)) };
            }
            unsafe {
                alloc::dealloc(self.values_ptr.as_ptr() as *mut u8,
                               Layout::array::<V>(self.capacity).unwrap())
            }
        }
    }
}