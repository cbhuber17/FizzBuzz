import java.util.Scanner;

public class FizzBuzz {
    /**
     * Main entry point of the program.
     *
     * This function is the main entry point of the program. It prompts the user to enter a positive integer number
     * using the `get_num` function, and generates a "FizzBuzz" sequence for that number using the `do_fb` function.
     * If an exception is thrown during the execution of the program, such as a NumberFormatException, the function
     * catches the exception and prints an error message to the console.
     *
     * @throws NumberFormatException if the entered value is not a non-negative integer
     */
    public static void main(String[] args) throws NumberFormatException {

        int iNum = 0;

        try {
            iNum = get_num();
            do_fb(iNum);
        } catch (Exception e) {
            System.out.println("ERROR: Input is not an integer or positive.");
        }
    }

    // -------------------------------------------------------------------
    /**
     * Prompts the user to enter a positive integer from the console, and returns the entered value.
     *
     * This function prompts the user to enter a positive integer number from the console. The function then
     * reads the entered value as an integer, and checks whether the value is positive using the `check_number`
     * function. If the entered value is not a positive integer, the function throws a NumberFormatException.
     *
     * @throws NumberFormatException if the entered value is not a positive integer
     * @return the positive integer entered by the user
     */
    public static int get_num() {

        System.out.print("Enter a positive integer number: ");

        Scanner in = new Scanner(System.in);

        // Will throw exception if number is float
        int iNum = in.nextInt();
        in.close();

        if(!check_number(iNum))
            throw new NumberFormatException();

        return iNum;
    }

    // -------------------------------------------------------------------

    /**
     * Checks if a given integer is negative.
     *
     * This function returns true if the given integer is positive (i.e., greater than or equal to zero),
     * and false otherwise.
     *
     * @param iNum_ the integer to be checked for postitivity
     * @return true if the given integer is positive, false otherwise
     */
    public static boolean check_number(int iNum_)
    {
        if(iNum_ < 0) {
            return false;
        }

        return true;
    }

    // -------------------------------------------------------------------

    /**
     * Returns a string representation of a given integer, according to the "FizzBuzz" rules.
     *
     * The "FizzBuzz" rules state that:
     * - If the number is divisible by 3, the string "Fizz" is added to the output.
     * - If the number is divisible by 5, the string "Buzz" is added to the output.
     * - If the number is not divisible by either 3 or 5, the number itself is converted to a string and added to the output.
     *
     * @param iNum_ the integer to be converted to a string according to the "FizzBuzz" rules
     * @return a string representation of the given integer, according to the "FizzBuzz" rules
     */
    public static String do_single_fb(int iNum_) {

        String szOutput = "";

        if((iNum_ % 3) == 0)
        {
            szOutput += "Fizz";
        }
        if((iNum_ % 5) == 0)
        {
            szOutput += "Buzz";
        }
        if(szOutput == "")
        {
            szOutput = Integer.toString(iNum_);
        }

        return szOutput;
    }

    // -------------------------------------------------------------------

    /**
     * Prints a sequence of "FizzBuzz" strings to the console, for integers from 1 to a given number.
     *
     * This function calls the do_single_fb function to generate "FizzBuzz" strings for each integer
     * from 1 to the given number. The "FizzBuzz" strings are printed to the console.
     *
     * @param iNum_ the number of integers to generate "FizzBuzz" strings for
     */
    public static void do_fb(int iNum_) {

        for(int i=1; i<iNum_+1; i++) {
            System.out.println(do_single_fb(i));
        }
    } 
}
