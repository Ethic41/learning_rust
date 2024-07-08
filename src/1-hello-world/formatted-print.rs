/* 
 * =====[Bismillahirrahmanirrahim]=====
 * -*- coding: utf-8 -*-
 * @Date    : 2024-07-07 11:24:51
 * @Author  : Your Name (you@example.org)
 * @Link    : link
 * @Version : 1.0.0
 */


fn main(){
    println!("{} days", 31);

    println!("{0}, this is {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // justify
    println!("{number:>5}", number=12);

    // padding
    println!("{number:0<8}", number=18);
    println!("{number:0>5}", number=185);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

}
