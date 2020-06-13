//Write a program that prints the numbers from 1 to 100. 
//But for multiples of three print “Fizz” instead of the number 
//and for the multiples of five print “Buzz”. 
//For numbers which are multiples of both three and five print “FizzBuzz”
#![allow(dead_code)]

pub fn match_fizz_buzz() {
    for num in 1..101 {
        match (num % 3, num % 5) {
            (0, 0) => println!("fizz-buzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", num)
        }
    };
}

pub fn map_fizz_buzz() {
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

pub fn one_println_fizz_buzz() {
    for num in 1..101 {
        let result = if num % 3 == 0 && num % 5 == 0 {
            "fizz-buzz"
        } else if num % 5 == 0 {
            "buzz"
        } else if num % 3 == 0 {
            "fizz-buzz"
        } else {
            //&*num.to_string()
            //format!("{}", num).into_string()
            ""
        };
        println!("{:?}", result)
    }
}

pub fn naive_imperative_fizz_buzz() {
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
