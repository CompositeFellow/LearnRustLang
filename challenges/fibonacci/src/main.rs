use std::io;

//General Functions
fn input_to_str() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    return input.trim().to_string()
}

fn get_n() -> u32{
    let num: String = input_to_str();
    match num.parse::<u32>(){
       Ok(n) => n,
       Err(_n) => {
        println!("Please enter a positive integer");
        get_n()
       }
    }
}


fn main() {
    println!("Math is fun");
    println!("Let's go through fibonacci.  Tell me a value for n");
    let n: u32 = get_n();

    fibonacci(n);
}

fn fibonacci(nth:u32) {
    //short circuit 
    if nth < 2 {
        println!("The first two numbers of fibonacci are 0 and 1. This wll be more fun if you pick a bigger number.");
    } else {
        let mut fibonacci_numbers = vec![0,1,1];
        for num in 2..nth +1 {
            let first: usize = (num ) as usize;
            let second: usize= (num-1) as usize;
            let new_num = fibonacci_numbers[first] + fibonacci_numbers[second];
            fibonacci_numbers.push(new_num);
        }
        println!("Fibonacci to n={} is  {:?}", nth, fibonacci_numbers);
    }
}

