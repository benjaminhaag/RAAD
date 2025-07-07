

mod raad;


fn main() {
    let mut vec = Vec::new();
    println!("{}, {}", vec.len(), vec.capacity());
    vec.push(raad::Option::Some(1));
    println!("{}, {}", vec.len(), vec.capacity());
    vec.push(raad::Option::Some(2));
    println!("{}, {}", vec.len(), vec.capacity());
    vec.push(raad::Option::Some(3));
    println!("{}, {}", vec.len(), vec.capacity());

    vec.extend([raad::Option::Some(4), raad::Option::Some(5)]);
    println!("{}, {}", vec.len(), vec.capacity());
    vec.extend([raad::Option::Some(6), raad::Option::Some(7)]);
    vec.extend([raad::Option::Some(8), raad::Option::Some(9)]);
    println!("{}, {}", vec.len(), vec.capacity());

    //let m = vec[2];

    for e in &vec {
        println!("{e:?}");
    }

    println!("{:?}", vec.pop());

    let vec1 = Vec::from([1, 2, 3]);
    let vec2 = vec!([1, 2, 3]);
    let vec3 = vec![0; 5];
    let mut vec4: Vec<u8> = Vec::with_capacity(5);
    vec4.resize(5, 0);
    vec4.shrink_to_fit();
    vec4.reserve(100); 
    // Guarantees capacity for at least 100 elements.
    // The length stays at 0!

    vec4.reserve_exact(100); 
    // Same, but skips over-allocating extra space.
    // (Rust's default `reserve` may allocate slightly more to avoid frequent reallocations.)

    println!("{}, {}", vec4.len(), vec4.capacity());

    let arr = [1, 2, 3]; // stack
    let boxed: Box<i32> = Box::new([1, 2, 3]);
}

