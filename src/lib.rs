pub mod option;
pub mod vec;

pub use option::Option;
pub use vec::Vec;



#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn new_vec_is_empty() {
        let vec: vec::Vec<i32> = vec::Vec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }

    #[test]
    fn reserve_exact_reallocates_memory() {
        let mut vec: vec::Vec<i32> = vec::Vec::new();
        let old_ptr = vec.as_ptr();
        vec.reserve_exact(20);
        let new_ptr = vec.as_ptr();
        assert_ne!(new_ptr, old_ptr);
    }

    #[test]
    fn reserve_exact_increases_capacity() {
        let mut vec: vec::Vec<i32> = vec::Vec::new();
        vec.reserve_exact(20);
        assert_eq!(vec.capacity(), 20);
    }

    #[test]
    fn reserve_exact_increases_capacity_once() {
        let mut vec: vec::Vec<i32> = vec::Vec::new();
        vec.reserve_exact(30);
        assert_eq!(vec.capacity(), 30);
        vec.reserve_exact(20);
        assert_eq!(vec.capacity(), 30);
    }

    #[test]
    fn reserve_increases_capacity() {
        let mut vec: vec::Vec<i32> = vec::Vec::new();
        vec.reserve(1);
        assert!(vec.capacity() >= 1);
    }

    #[test]
    fn push_pushes_one_element() {
        let mut vec = vec::Vec::new();
        let mut rng = rand::rng();
        let value: i32 = rng.random_range(0..1000);
        vec.push(value);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn get_returns_pushed_item() {
        let mut vec = vec::Vec::new();
        let mut rng = rand::rng();
        let value: i32 = rng.random_range(0..1000);
        vec.push(value);
        assert_eq!(vec.get(0), Some(value));
    }

    #[test]
    fn first_push_increases_capacity() {
        let mut vec = vec::Vec::new();
        let mut rng = rand::rng();
        let value: i32 = rng.random_range(0..1000);
        vec.push(value);
        assert!(vec.capacity() >= 1);
    }

}
