// Read input via console
const readline = require("readline");

// -------------------------------------------------------------------
// GLOBALS

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

// -------------------------------------------------------------------
// Functions

/**
 * The main function of the program, which retrieves a positive integer from the user
 * and performs the FizzBuzz calculation on the numbers from 1 to the input number.
 *
 * The function uses the `getNum` function to ask the user to input a positive integer,
 * and then calls the `doFB` function to perform the FizzBuzz calculation on the numbers
 * from 1 to the input number. If an error occurs while retrieving the input number or
 * performing the FizzBuzz calculation, the function logs an error message to the console
 * and exits the program with a status code of 1.
 */
async function main() {
  try {
    let num = 0;
    num = await getNum();
    doFB(num);
  } catch (err) {
    console.log(`${err}`);
    process.exit(1);
  }
}

// -------------------------------------------------------------------

/**
 * Asynchronously gets a positive integer number from the user via the command line interface.
 *
 * The function uses the `promptUser` function to ask the user to input a positive integer number.
 * It then checks if the input is a valid integer using the `checkNum` function. If the input is
 * not a valid integer, the function throws an error. Otherwise, it converts the input to an
 * integer and returns it.
 *
 * @throws {string} An error message if the user's input is not a positive integer.
 * @returns {Promise<number>} A promise that resolves with the user's input as a positive integer.
 */
async function getNum() {
  let num = await promptUser();
  rl.close();

  if (!checkNum(num)) throw "ERROR: Number is not positive or integer.";

  num = Number(parseInt(num, 10));

  return num;
}

// -------------------------------------------------------------------

/**
* Asks the user to input a positive integer number via the command line interface.
*
* The function uses the `readline` module to prompt the user to enter a positive integer
* number. It returns a `Promise` that resolves with the user's input as a string.
*
* @returns {Promise<string>} A promise that resolves with the user's input as a string.
*/
const promptUser = function () {
  return new Promise((resolve, reject) => {
    rl.question("Enter a positive integer number: ", (answer) => {
      resolve(answer);
    });
  });
};

// -------------------------------------------------------------------

/**
 * Checks if a given input is a valid integer.
 *
 * The function first checks if the input is a valid number and not a float (decimal),
 * returning `false` if it is. Then it converts the input to an integer and checks if it
 * is negative, returning `false` in that case. Finally, it uses `Number.isInteger()` to
 * check if the input is an integer, returning `true` or `false` accordingly.
 *
 * @param {*} num The input to check for integer validity.
 * @returns {boolean} `true` if the input is a valid integer, `false` otherwise.
 */
const checkNum = function (num) {
  // This is a check for float
  if (!isNaN(num) && num.toString().indexOf(".") != -1) return false;

  num = Number(parseInt(num, 10));

  // Check for negative number
  if (num < 0) return false;

  return Number.isInteger(num);
};

// -------------------------------------------------------------------

/**
 * Returns a string based on the input number, following the FizzBuzz game rules.
 *
 * If the number is divisible by 3, the output string contains "Fizz".
 * If the number is divisible by 5, the output string contains "Buzz".
 * If the number is not divisible by 3 or 5, the output string contains the number itself.
 *
 * @param {number} num The input number to evaluate.
 * @returns {string} The output string based on the FizzBuzz rules.
 */
const doSingleFB = function (num) {
  let output = "";

  if (num % 3 == 0) {
    output += "Fizz";
  }
  if (num % 5 == 0) {
    output += "Buzz";
  }
  if (output == "") {
    output = `${num}`;
  }

  return output;
};

// -------------------------------------------------------------------

/**
 * Prints the output of the FizzBuzz game for a given range of numbers.
 *
 * For each number from 0 to num (inclusive), the function calls the `doSingleFB` function
 * and prints the resulting output string to the console.
 *
 * @param {number} num The maximum number to evaluate (inclusive).
 * @returns {undefined} The function does not return a value.
 */
const doFB = function (num) {
  for (let i = 0; i < num + 1; i++) {
    console.log(doSingleFB(i));
  }
};

// -------------------------------------------------------------------

// Run the program
main();
