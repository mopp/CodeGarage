fn fizzbuzz(n: usize) -> String
{
    match n {
        _ if (n % 15) == 0 => "FizzBuzz".to_string(),
        _ if (n % 3)  == 0 => "Fizz".to_string(),
        _ if (n % 5)  == 0 => "Buzz".to_string(),
        _                  => n.to_string(),
    }
}


fn main()
{
    for n in 1..101 {
        print!("{} ", fizzbuzz(n));
    }
    println!("");
}
