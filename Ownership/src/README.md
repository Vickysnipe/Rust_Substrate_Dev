Implement a basic program that uses ownership concepts

In this task, i created a simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program takes two strings as input, concatenate them, and then print the result without violating any ownership rules.

Create a function called concatenate_strings that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.
Inside the concatenate_strings function, create a new String called result. Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.

Return the result string from the function

in the main function, i created two String variables, string1 and string2, and initialized them with appropriate values.
Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices). Store the result in a new variable called concatenated_string.

Printed the concatenated_string variable to the console.