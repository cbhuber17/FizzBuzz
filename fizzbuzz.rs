fn main() {
    let n = get_number();

    for i in 1..n + 1 {
        println!("{}", do_single_fb(i));
    }
}

// ----------------------------------------------------------------------------------------------

fn get_number() -> u32 {
    println!("Enter a positive integer number: ");

    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).unwrap();

    check_number(&num_str)
}

// ----------------------------------------------------------------------------------------------

fn check_number(num_str: &String) -> u32 {
    match num_str.trim().parse::<u32>() {
        Ok(number) => number,
        Err(_) => {
            panic!("Error: {} is not an integer. Exiting...", num_str.trim());
        }
    }
}

// ----------------------------------------------------------------------------------------------

fn do_single_fb(n: u32) -> String {
    let mut output: String = String::from("");

    if n % 3 == 0 {
        output += "Fizz";
    }
    if n % 5 == 0 {
        output += "Buzz";
    }
    if output == "" {
        output = n.to_string();
    }

    output
}

// ----------------------------------------------------------------------------------------------

// UNIT TESTING

#[cfg(test)]
mod test {
    use super::do_single_fb;

    #[test]
    fn test_fb() {
        assert_eq!(do_single_fb(1), "1");
        assert_eq!(do_single_fb(2), "2");
        assert_eq!(do_single_fb(3), "Fizz");
        assert_eq!(do_single_fb(4), "4");
        assert_eq!(do_single_fb(5), "Buzz");
        assert_eq!(do_single_fb(15), "FizzBuzz");
    }
}
