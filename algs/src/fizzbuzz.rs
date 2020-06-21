//Write a program that prints the numbers from 1 to 100. 
//But for multiples of three print “Fizz” instead of the number 
//and for the multiples of five print “Buzz”. 
//For numbers which are multiples of both three and five print “FizzBuzz”
#![allow(dead_code)]

pub fn match_several_println() {
    for num in 1..101 {
        match (num % 3, num % 5) {
            (0, 0) => println!("fizz-buzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", num)
        }
    };
}

pub fn match_one_println() {
    for num in 1..101 {
        let s = match (num % 3, num % 5) {
            (0, 0) => "fizz-buzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            _ => num.to_string()
        };
        println!("{}", s)
    };
}

pub fn map_default() {
    (1..101).map(|num| {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} fizz-buzz", num)
        } else if num % 5 == 0 {
            println!("{} buzz", num)
        } else if num % 3 == 0 {
            println!("{} fizz", num)
        } else {
            println!("{}", num)
        }
    }).collect()
}

pub fn map_match() {
    (1..101).map(|num| {
        match (num % 3, num % 5) {
            (0, 0) => println!("{} fizz-buzz", num),
            (0, _) => println!("{} fizz", num),
            (_, 0) => println!("{} buzz", num),
            _ => println!("{}", num)
        }
    }).collect()
}

pub fn one_println() {
    for num in 1..101 {
        let result = if num % 3 == 0 && num % 5 == 0 {
            "fizz-buzz".to_string()
        } else if num % 5 == 0 {
            "buzz".to_string()
        } else if num % 3 == 0 {
            "fizz-buzz".to_string()
        } else {
            num.to_string()
        };
        println!("{}", result)
    }
}

pub fn naive_imperative() {
    for num in 1..101 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} fizz-buzz", num)
        } else if num % 5 == 0 {
            println!("{} buzz", num)
        } else if num % 3 == 0 {
            println!("{} fizz-buzz", num)
        } else {
            println!("{}", num)
        }
    };
}
