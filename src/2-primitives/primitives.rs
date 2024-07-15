/* =====[Bismillahirrahmanirrahim]=====
 * -*- coding: utf-8 -*-
 * @Date    : 2024-07-08 19:55:46
 * @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 * @Link    : link
 * @Version : 1.0.0
 */

 use std::mem;


fn main(){
    // variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation
    let another_integer: i32 = 256; // regular annotation

    // or a default will be used
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // a mutable's variable value can be changed
    let mut mutable: i32 = 12; // mutable i32
    mutable = 21;

    // but it's type can be changed
    // mutable = true

    // literals and operators
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Tuples
    let long_tuple = (1u8, 2u16, 3u32, 'a', "ball", true);

    // extracting values from tuples
    println!("first value: {}", long_tuple.0);
    println!("Second value: {}", long_tuple.1);
    println!("Fifth value: {}", long_tuple.4);

    // Arrays and Slices

    // Fixed-sized
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of element in the array.
    println!("Number of elements in the array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));


}

// Tuples

fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}