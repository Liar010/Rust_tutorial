fn main() {
    /* if-else fizzbuzz */
    for i in 1..=100{
        if (i % 15) == 0 {
            println!("FizzBuzz");
        } else if (i % 5) == 0 {
            println!("Buzz");
        } else if (i % 3) == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
    /* match fizzbuzz */
    for i in 1..=100{
        match i {
            i if (i % 15) == 0 => println!("FizzBuzz"),
            i if (i % 5) == 0 => println!("Buzz"),
            i if (i % 3) == 0 => println!("Fizz"),
            _ => println!("{}", i),
        }
    }
}