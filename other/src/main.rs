#![allow(dead_code, unused_assignments, unused_variables)]

use std::ops::Add;

fn main() {
    // let mut s = String::from("hello");
    // s = takes_ownership(s);
    // println!("this:{}", s);
    //
    // let x = 5;
    // makes_copy(5);
    // println!("here:{}", x)
    let mut s1 = String::from("hello");

    let len= calculate_length(&mut s1);
    //println!("{}", s2)
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("11");
    let len = s.len();
    s.len()
}

fn takes_ownership(mut some_string: String) { // some_string comes into scope
    println!("{}", some_string);
    some_string.push_str("1")
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// fn reversed_array_parameter(arr: [i32; 4]) -> Vec<i32> {
//     //let temp = arr.iter().rev().;
//     //temp
// }

fn str_cpy() {
    let s1 = "hello";
    let s2 = s1;
    println!("{}", s1)
}

fn temp_3() -> String {
    let mut hello_str = String::from("Hello, ");
    hello_str.push_str("World");
    hello_str = hello_str.add("111").add("22");
    hello_str
    //let mut temp_2 = "str";
    //temp_2 = "new";

    //temp_2
}

fn reversed_array() {
    let arr = [10, 20, 30];

    for element in arr.iter().rev() {
        println!("{}", element)
    }

    println!("{:?}", arr)
}

fn tmp_1() -> i32 {
    let y = 6;
    y
}

fn temp_2() -> String {
    let x = 1;
    let mut x = String::from("temp_str");
    x = String::from("new_str");
    x
}

const TMP: i32 = 32;

fn temp_fn_one() -> i32 {
    let mut x = 5;
    x = 6;
    x
}

fn functions() -> i32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    10
}