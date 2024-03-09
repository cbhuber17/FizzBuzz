/// The main entry point of the program.
///
/// Reads a positive integer from the user, then applies the FizzBuzz logic to numbers from 1 to the entered number.
/// Each number is processed individually and printed along with its FizzBuzz result.
///
/// # Examples
///
/// ```
/// // Simulate user input "15"
/// // Output:
/// // 1
/// // 2
/// // Fizz
/// // 4
/// // Buzz
/// // Fizz
/// // 7
/// // 8
/// // Fizz
/// // Buzz
/// // 11
/// // Fizz
/// // 13
/// // 14
/// // FizzBuzz
fn main() {
    let n = get_number();

    for i in 1..n + 1 {
        println!("{}", do_single_fb(i));
    }
}

// ----------------------------------------------------------------------------------------------

/// Prompts the user to enter a positive integer number from standard input and parses it.
///
/// # Panics
///
/// Panics if the input cannot be parsed into a valid unsigned 32-bit integer.
///
/// # Returns
///
/// The parsed unsigned 32-bit integer entered by the user.
///
/// # Examples
///
/// ```
/// // Simulate user input "42"
/// assert_eq!(get_number(), 42);
/// ```
fn get_number() -> u32 {
    println!("Enter a positive integer number: ");

    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).unwrap();

    check_number(&num_str)
}

// ----------------------------------------------------------------------------------------------

/// Parses a string into an unsigned 32-bit integer after trimming whitespace.
///
/// # Arguments
///
/// * `num_str` - A reference to the string to be parsed into an integer.
///
/// # Panics
///
/// Panics if the string cannot be parsed into a valid unsigned 32-bit integer.
///
/// # Returns
///
/// The parsed unsigned 32-bit integer.
///
/// # Examples
///
/// ```
/// assert_eq!(check_number(&String::from("123")), 123);
/// assert_eq!(check_number(&String::from("  456  ")), 456);
/// ```
fn check_number(num_str: &String) -> u32 {
    match num_str.trim().parse::<u32>() {
        Ok(number) => number,
        Err(_) => {
            panic!(
                "Error: {} is not a (positive) integer. Exiting...",
                num_str.trim()
            );
        }
    }
}

// ----------------------------------------------------------------------------------------------

/// Implements the FizzBuzz logic for a single number.
///
/// # Arguments
///
/// * `n` - The number to apply the FizzBuzz logic to.
///
/// # Returns
///
/// A `String` containing the FizzBuzz result for the given number.
///
/// # Examples
///
/// ```
/// assert_eq!(do_single_fb(3), "Fizz");
/// assert_eq!(do_single_fb(5), "Buzz");
/// assert_eq!(do_single_fb(15), "FizzBuzz");
/// assert_eq!(do_single_fb(7), "7");
/// ```
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
