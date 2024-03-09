# FizzBuzz

# Coding Club FizzBuzz

The second challenge as part of the [Coding Club](https://teams.microsoft.com/l/team/19%3aQjxVFz5CwSOZ0nKM9tUXDfMh5xLaqD_Jn5hwdVgqEEs1%40thread.tacv2/conversations?groupId=1a34467b-4e60-461c-a845-202847e59a20&tenantId=1b16ab3e-b8f6-4fe3-9f3e-2db7fe549f6a) is to create a program in any language to do the following:

FizzBuzz is a classical programming interview question to quickly find out if the applicant can think through logical conditions. Write a program that prints integers one-to-N. Any integers divisible by three should be replaced with “Fizz”, integers divisible by five as “Buzz” and integers divisible by both three and five as “FizzBuzz”.

Example:

```
Enter a number: 15

1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

If you would like to learn more about this problem, watch this video: https://www.youtube.com/watch?v=QPZ0pIK_wsc

# Installation/Usage

The task was completed in 5 different programming languages:

- Python
- C++
- Java
- JavaScript
- Rust

See below for installation and executing the code written for these files.

## Python

Install Python 3.10 or 3.11 here:

https://www.python.org/downloads/release/python-3100/

https://www.python.org/downloads/release/python-3110/

Open a command window in the same directory as [fizzbuzz.py](fizzbuzz.py)/[test_fizzbuzz.py](test_fizzbuzz.py) and run the program (and corresponding unit testing) as follows:

```
python fizzbuzz.py
python test_fizzbuzz.py
```

## C++

For VS Code, follow these instructions:

1. Download MSYS2 here: https://www.msys2.org/

2. Install the downloaded MSYS2 64-bit.

3. Run the MSYS2 64-bit after installation (will ask at the end of the installation wizard.)

4. MSYS2 resembles a command terminal. The required packages are to be installed with the following command (case senstitve!)

```
pacman -Syu
```

Respond with `'y'` to proceed with installation.

5. Close the MSYS2 terminal afterwards.

6. Hit the Windows start button and type `MSYS2 MSYS` and this will open another command window.

7. The required packages are to be installed with the following command (case senstitve!)

```
pacman -Su
```

Respond with `'y'` to proceed with installation.

8. The C++ package installation is required with the following command (case senstitve!)

```
pacman -S mingw-w64-x86_64-gcc
```

Respond with `'y'` to proceed with installation.

9. After installation, confirm installation with the following command:

```
gcc --version
g++ --version
```

10. Install a C++ debuger with the following command.

```
pacman -S mingw-w64-x86_64-gdb
```

Respond with `'y'` to proceed with installation.

11. After installation, confirm installation with the following command:

```
gdb --version
```

12. Close all instances of VS Code.

13. Set the path to the location where MINGW was installed:
    a. Hit the windows start button and type "Environment" and choose "Edit the system environment variables."
    b. Select "Environment Variables..." at the bottom.
    c. Under System variables, select "Path" and click "Edit.
    d. Click "New" and add the installed directory at `C:\msys64\mingw64\bin` .
    e. Click OK.

14. Open any new command window (not any existing ones that are already open), and type:

```
gcc --version
g++ --version
gdb --version
```

Each of these should successfully report their version.

15. Open VS Code, click on Extensions and search for C++ and install the Microsoft C++ extension.

16. Also install the "Code Runner" extension by Jun Han.

17. Close all instances of VS Code.

18. Open VS Code and go to File > Preferences > Settings > Extensions and scroll down to "Run Code Configurations" and select the tick box "Run in Terminal".

19. Open [fizzbuzz.cpp](fizzbuzz.cpp) in VS code (make sure [fizzbuzz.h](fizzbuzz.h) is present as well). Right click and select "Run Code". This will run in the VS Code terminal, and also create a `"fizzbuzz.exe"` to be run outside in a separate terminal if desired.

## Java

For VS Code, follow these instructions:

1. Close all instances of VS Code.

2. Download the latest JDK here: https://www.oracle.com/java/technologies/downloads/#jdk19-windows (x64 Installer is sufficent)

3. Open a new command window and type:

```
javac --version
```

and ensure a version was successfully returned.

4. Open VS Code, and open [fizzbuzz.java](fizzbuzz.java). Locate the function `public static void main` near the top of the file. There should be a `"Run|Debug"` statement near this function, click "Run" to launch the Java code.

## JavaScript

1. Close all instances of VS code.

2. Go to https://nodejs.org/en/ and download the LTS version.

3. Open a new command window and type:

```
node --version
```

and ensure a version was successfully returned.

4. Open a command window in the same directory as [fizzbuzz.js](fizzbuzz.js) and type the following to run the program:

```
node fizzbuzz.js
```

## Rust

1. Download Rust Cargo at https://www.rust-lang.org/tools/install.

2. Open a new command window and type:

```
cargo new rust-fizz-buzz
```

3. Copy the contents [fizzbuzz.rs](fizzbuzz.rs) to src/main.rs.

4. Run the program with the following command:

```
cargo run
```

5. Perform unit testing with the following command:

```
cargo test
```
