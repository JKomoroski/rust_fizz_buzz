fn main() {
    let mut x = 1; 

    while x <= 100 {
        print_fizz_buzz(x);
        x += 1;
    }

    fn print_fizz_buzz(number:i64) {
        if number % 3 == 0 || number % 5 == 0 {
            if number % 3 == 0 {
                print!("Fizz");
            }
        
            if number % 5 == 0 {
                print!("Buzz");
            }

            println!("");
            return;
        }

        println!("{}", number);
    }
}
