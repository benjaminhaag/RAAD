use std::{
    alloc::{self, Layout},
};

pub struct Vec {
    len: usize,
    capacity: usize,
    ptr: *mut i32,
}

impl Vec {
    pub fn new() -> Self {
        unsafe {
            Self {
                len: 0,
                capacity: 0,
                ptr: alloc::alloc(Layout::array::<i32>(0).unwrap()) as *mut i32,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.len {
            return None;
        }
        unsafe {
            Some(self.ptr.add(index).read())
        }
    }

    pub fn reserve_exact(&mut self, size: usize) {
        let required = self.len + size;
        if required <= self.capacity { return; }

        self.ptr = unsafe {
            let new_ptr = alloc::alloc(alloc::Layout::array::<i32>(required).unwrap()) as *mut i32;
            for i in 0..self.len {
                let val = self.ptr.add(i).read();
                new_ptr.add(i).write(val);
            }
            alloc::dealloc(self.ptr as *mut u8,  Layout::array::<i32>(self.capacity).unwrap());
            new_ptr
        } as *mut i32;
        self.capacity = required;
    }

    pub fn reserve(&mut self, size: usize) {
        let required = self.len + size;
        if required <= self.capacity { return; }
        if self.capacity == 0 {
            self.reserve_exact(4);
        } else if self.capacity.is_power_of_two() {
            self.reserve_exact(required.next_power_of_two() - self.len);
        } else {
            self.reserve_exact(size);
        }
    }

    pub fn push(&mut self, value: i32) {
        self.reserve(1);
        unsafe {
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
    }

    pub fn as_ptr(&self) -> *const i32 {
        self.ptr
    }

}

impl Drop for Vec {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8,  Layout::array::<i32>(self.capacity).unwrap());
        }
    }
}


