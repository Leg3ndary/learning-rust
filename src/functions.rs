pub fn check_fizzbuzz(number: &i32) {
    if number % 15 == 0 {
        println!("FizzBuzz");
    } else if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", number);
    }
}

pub fn multiply_nums(num1: i32, num2: i32) -> i32 {
    num1 * num2
}