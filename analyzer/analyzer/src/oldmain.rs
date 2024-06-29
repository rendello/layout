 
#![feature(portable_simd)]
use std::simd::{u8x16};
use std::simd::cmp::SimdPartialEq;

const S: &str = "Hello, world\nMy name is Gaven.";

fn main() {
    let text = u8x16::load_or_default(S.as_bytes());

    let spaces = u8x16::splat(b' ').simd_eq(text);
    let newlines = u8x16::splat(b'\n').simd_eq(text);

    let result = spaces | newlines;

    println!("{:?}", result);
}