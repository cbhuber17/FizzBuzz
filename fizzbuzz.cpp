#include <iostream>
#include <stdio.h>
#include <stdexcept>
#include "fizzbuzz.h"

using namespace std;

int main()
{
    int iNum = 0;

    try
    {
        iNum = get_num();
        do_fb(iNum);
    }
    catch(const invalid_argument& e)
    {
        printf("ERROR: %s", e.what());
    }
    
    return 0;
}

// -------------------------------------------------------------------

/**
 * Checks if a string contains only numeric characters.
 *
 * This function iterates over each character in the input string and checks if it is a digit.
 *
 * @param szStr_ The string to check.
 * @return `true` if the string contains only numeric characters, `false` otherwise.
 *
 * @note This function considers an empty string to be numeric.
 */
bool check_number(string szStr_)
{
    for (int i = 0; i < szStr_.length(); i++)
    {
        if (!isdigit(szStr_[i]))
            return false;
    }
   return true;
}

// -------------------------------------------------------------------

/**
 * Prompts the user to enter a numeric string, then converts it to an integer.
 *
 * This function prompts the user to enter a string and reads it from standard input using scanf.
 * It then converts the string to an integer using stoi, and returns the result.
 *
 * @throws std::invalid_argument if the input string is not a valid integer or is not positive.
 * @return The integer value entered by the user.
 *
 * @note This function does not handle input errors gracefully, and may exit the program if invalid input is provided.
 */
int get_num()
{
    int iResult = 0;
    char cTemp[128];
    string szNum = "";

    printf("Enter a positive integer number: ");

    iResult = scanf("%127s", &cTemp);

    szNum = cTemp;

    if(iResult != 1 || !check_number(szNum))
        throw invalid_argument("Is not an integer or positive");
    else
        return stoi(szNum);
}

// -------------------------------------------------------------------

/**
 * Computes the FizzBuzz string for a single number.
 *
 * This function takes an input integer and computes the FizzBuzz string for that number.
 * If the number is divisible by 3, "Fizz" is appended to the output string.
 * If the number is divisible by 5, "Buzz" is appended to the output string.
 * If the number is not divisible by either 3 or 5, the input number is converted to a string and returned.
 *
 * @param iNum_ The number to compute the FizzBuzz string for.
 * @return The computed FizzBuzz string.
 */
string do_single_fb(int iNum_)
{
    string szOutput = "";

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
        szOutput = to_string(iNum_);
    }

    return szOutput;
}

// -------------------------------------------------------------------

/**
 * Computes and prints the FizzBuzz string for numbers 1 to the given input.
 *
 * This function takes an input integer and computes the FizzBuzz string for all numbers from 1 to the input.
 * For each number, the `do_single_fb` function is called to compute the corresponding FizzBuzz string,
 * which is then printed to standard output using printf.
 *
 * @param iNum_ The maximum number to compute the FizzBuzz string for.
 */
void do_fb(int iNum_)
{
    string szOutput = "";
    for(int i=1; i<iNum_+1; i++)
    {
        szOutput = do_single_fb(i).data();
        printf("%s\n", szOutput.c_str());
    }
}