use std::vec::Vec;

extern crate rand;

use rand::Rng;
use std::intrinsics::write_bytes;
use std::fmt::{Display, Formatter};

mod practice_57;

use practice_57::count_str::count_str;


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
//    debug_practice();
//    display_practice();
//    formatting();
//    tuple_practice();
//    enums_practices()
//    conversion_practice();
//    assign_practice();
//    lambda_practice()

    count_str();
}

fn num() {
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
}


fn debug_practice() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "peter";
    let age = 27;
    let peter = Person { name, age };

    // pretty print
    println!("{:#?}", peter);
}

fn display_practice() {
    use std::fmt;

    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let s = Structure(1);
    println!("{}", s);

    let vector = vec![1, 2, 3, 4, 5];
    println!("{:?}", vector);
}

fn formatting() {
    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    for city in [City { name: "Dublin", lat: 53.347778, lon: -6.259722 }].iter() {
        println!("{}", *city);
    }
}

fn tuple_practice() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;

        /* Array, Vector, and Slice */
        (boolean, integer)
    }

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("long tuple {:?}", long_tuple);
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple {:?}", too_long_tuple);


    //named struct
    struct Matrix(f32, f32, f32, f32);

    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Matrix({:?}, {:?}, {:?}, {:?})", self.0, self.1, self.2, self.3)
        }
    }

    let mtx = Matrix(1.2, 1.0, 2.0, 3.5);
    println!("{}", mtx);
}

fn sequences() {
    /// Array, Vector, and Slice
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


    // pop data
    vec2 = vec![1, 2, 3];
    assert_eq!(vec2.pop().unwrap(), 3);
    assert_eq!(vec2, vec![1, 2]);

    // insert ele in vec (index, value)
    vec2.insert(1, 99);
    assert_eq!(vec2, vec![1, 99, 2]);


    /* slice is the reference of array or vector */
    // new slice
    // # 1
    let _s = [1, 2, 3, 4, 5];
    let slice_arr = &_s;

    // # 2
    let slice_arr = &[1, 2, 3, 4, 5];
    // # 3 allow to write
    let slice_arr = &mut [1, 2, 3, 4, 5];

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
}

fn enums_practices() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            WebEvent::KeyPress(c) => println!("press {}", c),
            WebEvent::Paste(s) => println!("paste {}", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x, y = {}, {}", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("My text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);

    /* use */

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    use Status::{Poor, Rich};
    use Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;

    match status {
        Rich => println!("rich!"),
        Poor => println!("poor"),
    }
}

fn conversion_practice() {
    /* built-in casting "as"   */
    let decimal: f32 = 65.4321;
    let _int = decimal as u8;
    let character = _int as char;

    println!("Casting {} -> {} -> {} ", decimal, _int, character);


    let decimal_f64 = decimal as f64;


    /* from and into */

    let my_str = "Hello";
    let my_string = String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);
    let _int = 5;
    let num: Number = _int.into();
    println!("My number is {:?}", num);


    /* To and from string */
    use std::string::ToString;

    struct Circle {
        radius: i32
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 7 };
    println!("{}", circle.to_string());

    let parsed_int: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let float_parsed = "10".parse::<f64>().unwrap();


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


fn lambda_practice() {
    use std::mem;

    let color = "Green";
    let print = || println!("color: {}", color);

    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {} ", count);
    };

    inc();
    inc();

    //let _reborrow = &mut count;
    inc();

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

//    consume();
    consume();




}


fn assign_practice() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Match {:?}!", i);
    }

    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}



























