use std::{ptr, mem, alloc::{self, Layout}};

#[derive(Debug)]
pub enum Option<T> {
None,
Some(T),
}

impl<T> Option<T> {
pub fn unwrap(self) -> T {
    self.expect("called `Option::unwrap()` on a `None` value")
}

pub fn expect(self, error: &str) -> T {
    match self {
        Option::Some(v) => v,
        Option::None => panic!("{}", error),
    }
}
}





pub struct Vec<T> {
len: usize,
capacity: usize,
content: *mut T,
}


impl<T> Vec<T> {

pub fn new() -> Self {
    Self {
        len: 0,
        capacity: 0,
        content: ptr::null_mut(),
    }
}

pub fn len(&self) -> usize {
    self.len
}

pub fn capacity(&self) -> usize {
    self.capacity
}

pub fn grow(&mut self) {
    let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
    let new_layout = Layout::array::<T>(new_capacity).unwrap();

    unsafe {
        let new_ptr = if self.capacity == 0 {
            alloc::alloc(new_layout)
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            alloc::realloc(self.content.cast(), old_layout, new_layout.size())
        };

        if new_ptr.is_null() {
            alloc::handle_alloc_error(new_layout);
        }

        self.content = new_ptr.cast();
        self.capacity = new_capacity;
    }
}

pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
        self.grow();
    }

    unsafe {
        self.content.add(self.len).write(value);
    }
    
    self.len += 1;
}

pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Option::Some(&*self.content.add(index)) }
        } else {
            Option::None
        }
    }

}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.content.add(i));
            }
            if self.capacity != 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.content.cast(), layout);
            }
        }
    }
}
