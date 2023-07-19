pub mod vector {
    use std::alloc::{Layout};
    use std::{alloc, mem, ptr};
    use std::ptr::NonNull;

    pub struct MyVector<T> {
        ptr: NonNull<T>,
        len: usize,
        capacity: usize,
    }

        impl<T> MyVector<T> {
            //TODO delete after
            pub fn testing_ptr(&mut self) -> T{
                unsafe{ptr::read(self.ptr.as_ptr().add(self.len))}
            }

        pub fn new() -> Self {
            let cap = if mem::size_of::<T>() == 0 { usize::MAX } else { 0 };
            Self {
                ptr: NonNull::dangling(),
                len: 0,
                capacity: cap,
            }
        }

        pub fn with_capacity(cap: usize) -> Self {
            let mut v: MyVector<T> = MyVector::new();
            while v.capacity < cap {
                v.grow();
            }
            v
        }

        pub fn capacity(&self) -> usize {
            self.capacity
        }

        pub fn get(&mut self, index:usize) -> Option<T>{
            if index >= self.len{
                None
            }else {
                unsafe {
                    Some(ptr::read(self.ptr.as_ptr().add(index)))
                }
            }
        }

        pub fn push(&mut self, elem: T) {
            if self.len == self.capacity { self.grow() }

            unsafe {
                ptr::write(self.ptr.as_ptr().add(self.len), elem)
            }

            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
            }
        }

        pub fn insert(&mut self, index: usize, elem: T) {
            assert!(index <= self.len, "Index out og bounds");
            if self.capacity == self.len {
                self.grow();
            }
            unsafe {
                ptr::copy(
                    self.ptr.as_ptr().add(index),
                    self.ptr.as_ptr().add(index + 1),
                    self.len - index,
                );
                ptr::write(self.ptr.as_ptr().add(index), elem);
                self.len += 1;
            }
        }

        pub fn remove(&mut self, index: usize) -> T {
            assert!(index < self.len, "index out pf bounds");
            unsafe {
                self.len -= 1;
                let elem = ptr::read(self.ptr.as_ptr().add(index));
                ptr::copy(
                    self.ptr.as_ptr().add(index + 1),
                    self.ptr.as_ptr().add(index),
                    self.len - index,
                );
                elem
            }
        }

        fn grow(&mut self) {
            assert_ne!(mem::size_of::<T>(), 0, "Capacity overflow");

            let (new_capacity, new_layout) = if self.capacity == 0 {
                (1, Layout::array::<T>(1).unwrap())
            } else {
                let new_capacity = 2 * self.capacity;
                (new_capacity, Layout::array::<T>(new_capacity).unwrap())
            };

            assert!(new_layout.size() <= isize::MAX as usize, "Allocation to large");

            let new_ptr = if self.capacity == 0 {
                unsafe { alloc::alloc(new_layout) }
            } else {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            };

            self.ptr = match NonNull::new(new_ptr as *mut T) {
                Some(s) => s,
                None => alloc::handle_alloc_error(new_layout)
            };
            self.capacity = new_capacity;
        }
    }

    impl<T> Drop for MyVector<T> {
        fn drop(&mut self) {
            if self.capacity != 0 {
                while let Some(_) = self.pop() {}
                let layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
                }
            }
        }
    }

    impl <T:Clone> MyVector<T>{
        pub fn resize(&mut self, new_len:usize, value: T){
            if new_len < self.len{
                for _ in 0..(self.len - new_len){
                    let _ = self.pop();
                }
            }else {
                for _ in 0..(new_len-self.len){
                    self.push(value.clone());
                }
            }
        }
    }
}
