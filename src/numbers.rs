fn numbers() {

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
    println!("isize min: {}, max: {}", isize::min_value(), isize::max_value());
    println!("usize min: {}, max: {}", usize::min_value(), usize::max_value());

    // float


    assert!(f64::abs((0.3 - 0.2) - 0.1) < 1e-10);
    assert!(f64::abs(1.0 - (1 as f64)) < 1e-10);

}