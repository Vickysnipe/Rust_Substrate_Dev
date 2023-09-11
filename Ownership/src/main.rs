//a function called concatenate_strings that takes two string slices as arguments
 // and returns a new String as the result

fn concatenate_strings(s1: &str, s2: &str) -> String {

    //new String called result to store the concatenated string

    let mut result = String::new();
    //the push_str() method to append the contents of the first and second input string slice s1 s2 to result

    result.push_str(s1);
    result.push_str(s2);

    //the result string from the function.

    result
}

fn main() {
    // Here i created two String variables, string1 and string2, and initialized them with appropriate values.

    let string1 = "Hello, World ";
    let string2 = "I am learning Rust";
    
    let concatenated_string = concatenate_strings(string1, string2);
    
    //here i printed the concatenated_string variable to the console

    println!("{}", concatenated_string);
}
