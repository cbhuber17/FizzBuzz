"""
Runs the FizzBuzz application: Asks the user for an integer
input and returns "Fizz" if divisible by 3, "Buzz" if divisible
by 5, "FizzBuzz" if divisible by both, and the integer otherwise.
"""

# ----------------------------------------------------------------------------------------------

def get_number() -> str:
    """
    Asks the user to input an integer number and returns it as a string.

    This function prompts the user to enter an integer number through the console, 
    and then returns the user input as a string. If the input is not a valid integer, 
    it will be returned as a string regardless.

    Returns:
    - A string that represents the integer number entered by the user.
    """

    n = input('Enter a postive integer number: ')
    return n

# ----------------------------------------------------------------------------------------------

def check_number(n) -> int:
    """Converts the input parameter to an integer and returns it if it is a valid integer.

    Args:
        n (Any): A value that can be cast to an integer.

    Returns:
        int: The integer value of the input parameter.

    Raises:
        SystemExit: If the input parameter cannot be cast to an integer.
    """

    try:

        # Check for float
        if isinstance(n, float):
            raise ValueError
        
        # Cast string to int
        n = int(n)

        # Check positive
        if n <= 0:
            raise ValueError
        
    except ValueError:
        print(f"ERROR: {n} is not an integer or positive.")
        raise

    return n

# ----------------------------------------------------------------------------------------------

def do_single_fb(n) -> str:
    """Returns a string containing 'Fizz' if n is divisible by 3, 'Buzz' if n is
    divisible by 5, 'FizzBuzz' if n is divisible by both 3 and 5, or the string
    representation of n if none of the above conditions are met.

    Args:
        n: An integer to check for divisibility by 3 and/or 5.

    Returns:
        A string containing 'Fizz', 'Buzz', 'FizzBuzz', or the string
        representation of n.
    """

    output = ''

    if n % 3 == 0:
        output += 'Fizz'
    if n % 5 == 0:
        output += 'Buzz'
    if not output:
        output = str(n)

    return output

# ----------------------------------------------------------------------------------------------

if __name__ == '__main__':

    n = get_number()

    n = check_number(n)

    for i in range(1, n+1):
        print(do_single_fb(i))
