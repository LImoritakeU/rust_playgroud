use std::vec::Vec;

extern crate rand;

use rand::Rng;

fn bubble_sort(slice: &mut Vec<i32>) -> Vec<i32> {
    let num = slice.len() - 1;
    let len = slice.len();

    for _ in 0..num {
        for index in 0..num {

            let x = slice[(len - 2) - index];
            let y = slice[(len - 1) - index];
            if x <= y {
                slice[(len - 2) - index] = x;
                slice[(len - 1) - index] = y;

            } else {
                slice[(len - 1) - index] = x;
                slice[(len - 2) - index] = y;
            }
        }
        println!("slice: {:?}", slice);
    }
    slice.to_vec()
}


fn main() {
    /* min and max */
    // signed int
    println!("i8 min: {}, max: {}", i8::min_value(), i8::max_value());
    println!("i16 min: {}, max: {}", i16::min_value(), i16::max_value());
    println!("i32 min: {}, max: {}", i32::min_value(), i32::max_value());
    println!("i64 min: {}, max: {}", i64::min_value(), i64::max_value());

    // unsigned int
    println!("u8 mun: {}, max: {}", u8::min_value(), u8::max_value());
    println!("u16 mun: {}, max: {}", u16::min_value(), u16::max_value());
    println!("u32 mun: {}, max: {}", u32::min_value(), u32::max_value());
    println!("u64 mun: {}, max: {}", u64::min_value(), u64::max_value());
    println!("u64 mun: {}, max: {}", u64::min_value(), u64::max_value());

    // variable-width
    println!(
        "isize min: {}, max: {}",
        isize::min_value(),
        isize::max_value()
    );
    println!(
        "usize min: {}, max: {}",
        usize::min_value(),
        usize::max_value()
    );

    // float
    assert!(f64::abs((0.3 - 0.2) - 0.1) < 1e-10);
    assert!(f64::abs(1.0 - (1 as f64)) < 1e-10);

    /* Array, Vector, and Slice */
    // new array
    let arr = ["eins", "zwei"];
    let mut array = [0.0; 10];

    // read array
    assert_eq!(arr[0], "eins");
    assert_eq!(arr[1], "zwei");

    // write array
    array[0] = 1.0;
    array[1] = 2.0;

    // iter read array
    for i in arr.iter() {
        println!("{}", i);
    }

    // iter write array
    let mut arr2 = [0; 10];
    for i in arr2.iter_mut() {
        *i = rand::thread_rng().gen_range(1, 100);
    }

    // new vector
    let vec = vec![1, 2, 3];
    let mut vec2 = Vec::new();

    // append data to vector
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);

    // read vector

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);

    // write vector
    vec2[0] = 0;
    vec2[1] = 1;
    vec2[2] = 2;

    // iter read vector
    // #1   by index
    for i in 0..(vec.len()) {
        println!("{}", vec[i]);
    }

    // #2   by iterator
    for ele in vec.iter() {
        println!("{}", ele);
    }

    // write vector
    // # 1 by index
    for i in 0..vec2.len() {
        vec2[i] = vec2[i] + vec2[i];
    }

    // #2 by iterator
    for ele in vec2.iter_mut() {
        *ele = *ele + *ele;
    }

    // get length of vector
    assert_eq!(vec.len(), 3);

    // pop data
    vec2 = vec![1, 2, 3];
    assert_eq!(vec2.pop().unwrap(), 3);
    assert_eq!(vec2, vec![1, 2]);

    // insert ele in vec (index, value)
    vec2.insert(1, 99);
    assert_eq!(vec2, vec![1, 99, 2]);

    // remove ele in vec
    assert_eq!(vec2.remove(1), 99);

    /* slice is the reference of array or vector */
    // new slice
    // # 1
    let _s = [1, 2, 3, 4, 5];
    let slice_arr = &_s;

    // # 2
    let slice_arr = &[1, 2, 3, 4, 5];
    // # 3 allow to write
    let slice_arr = &mut [1, 2, 3, 4, 5];


    /* practice: implement insertion sort, bubble sort, selection sort, bucket sort*/

    // bubble sort
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..10 {
        numbers.push(rng.gen_range(1, 100))
    }
    println!("{:?}", numbers);
    println!("bubble sort: {:?}", bubble_sort(&mut numbers));
}
