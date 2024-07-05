 
#![feature(portable_simd)]
use std::simd::{u8x16};
use std::simd::cmp::SimdPartialEq;

// const S: &str = "Hello, world\nMy name is Gaven.";

// fn main() {
//     let text = u8x16::load_or_default(S.as_bytes());

//     let spaces = u8x16::splat(b' ').simd_eq(text);
//     let newlines = u8x16::splat(b'\n').simd_eq(text);

//     let result = spaces | newlines;

//     println!("{:?}", result);
// }

//  #![feature(portable_simd)]

// use std::simd::{u8x16};
// use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};

// const S: &str = "Hello, world\nMy name is Gaven.";

// #[inline]
// fn simd_compare_range(bytes: &u8, lower: u8, upper: u8) {
//     assert!(lower < upper);

//     let input = u8x16::load_or_default(bytes);
//     let a = u8x16::splat(lower as u8);

//     let z_plus_1 = u8x16::splat((upper as u8) + 1);

//     let lt_97 = input.simd_lt(a);
//     let lt_123 = input.simd_lt(z_plus_1);

//     let not_lt_97 = !lt_97;
//     let t2 = not_lt_97 & lt_123;
// }

// fn main() {


//     // t2 now contains the result of the range check
//     println!("{:?}", t2.to_array());
// }



fn get_spaces(text: &str) -> Vec<u16> {
    let bytes: &[u8] = text.as_bytes();
    let mut buffer: Vec<u16> = Vec::new();

    for chunk in bytes.chunks(16) {
        let input = u8x16::load_or_default(&chunk);
        let space_mask = u8x16::splat(' ' as u8);

        let bm: u16 = input.simd_eq(space_mask).to_bitmask().try_into().unwrap();

        buffer.push(bm);
    };
    buffer
}

fn main() {
    let s = "Hello, world! It's me, Steven. Today, we're going to talk about code.";
    let res = get_spaces(&s);
    println!("{}", s);
    for v in res {
        print!("{:016b}", v);
    }
    println!("");
}