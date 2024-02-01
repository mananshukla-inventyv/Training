use std::io;

pub fn split_string() -> String {
    let mut _str1 = String::new();
    let mut _letter = String::new();
    // Read the input string from the user.
    println!("Enter string :");
    io::stdin()
        .read_line(&mut _str1)
        .expect("error while reading input");

    // Read the word to replace from the user.
    println!("Enter word to replace :");
    io::stdin()
        .read_line(&mut _letter)
        .expect("error while reading input");

    // Trim leading and trailing whitespaces from the input strings.
    let str1 = _str1.trim();
    let letter = _letter.trim();

    // Split the input string by the word to replace and join the substrings with underscores.
    let substrings: Vec<&str> = str1.split(letter).collect();
    let joined_result: String = substrings.join("_");

    // Return the final result.
    joined_result
}