#![allow(dead_code, unused_assignments)]
fn main() {
    temp_function();
    println!("{}", TMP)
}

const TMP: i32 = 32;

fn temp_function() -> i32 {
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