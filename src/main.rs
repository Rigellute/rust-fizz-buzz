fn main() {
    let mut num = 0;

    loop {
        num += 1;

        let is_fizz = num % 3 == 0;
        let is_buzz = num % 5 == 0;
        let is_fizz_buzz = is_fizz && is_buzz;
        if is_fizz_buzz  {
            println!("Fizzbuzz");
        } else if is_fizz {
            println!("Fizz");
        } else if is_buzz {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
        if num > 100 {
            break;
        }
    }
}
