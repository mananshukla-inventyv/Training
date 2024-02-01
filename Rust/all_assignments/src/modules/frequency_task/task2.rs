use std::collections::HashMap;
use std::io;
use crate::modules::type_collections::Task2Response;
pub use crate::modules::frequency_task::task2::string_analyzer as string_analyze;
pub fn string_analyzer() -> Task2Response {
    // initializing empty string for inputs
    let mut _str1 = String::new();
    let mut _str2 = String::new();
    //input strings
    println!("Enter string1 :");
    io::stdin()
        .read_line(&mut _str1)
        .expect("error while reading input");
    println!("Enter srting2 :");
    io::stdin()
        .read_line(&mut _str2)
        .expect("error while reading input");
    //triming the string as it contains
    let str1 = _str1.trim();
    let str2 = _str2.trim();
    // calling letter_count to get char and its count of a particular string
    let str1_count = letter_count(str1);
    let str2_count = letter_count(str2);
    // initializing vector of tuple which will store char and its count
    let mut letter_count_vec: Vec<(char, u8)> = Vec::new();
    let mut left_out: Vec<(char, u8)> = Vec::new();

    //looping on each element of vector to take letter and count
    for (&letter, &char_count_in_str1) in &str1_count {
        //if char is present in both the strings
        if let Some(&char_count_in_str2) = str2_count.get(&letter) {
            //get total frequency of chars in both the strings
            let total_count = char_count_in_str1 + char_count_in_str2;
            // if total is more than one push into letter_count_vec
            if total_count > 1 {
                letter_count_vec.push((letter, total_count));
            }
            //else push it into left_out vec
        } else {
            left_out.push((letter, 0));
        }
    }
    //sorting both the vectors in alphabetic order
    sort_vecs(&mut letter_count_vec);
    sort_vecs(&mut left_out);
    // return both the vectors in struct format

    Task2Response {
        letter_count_vec,
        left_out,
    }
}

/// Counts the occurrences of each character in a given string and returns a `HashMap`.
///
/// This function takes a string as an argument, converts it to lowercase, and counts the occurrences
/// of each character. The result is returned as a `HashMap` where characters are keys and counts are values.
///
/// # Arguments
///
/// * `str` - The input string to count the occurrences of characters.
///
/// # Returns
///
/// Returns a `HashMap` containing characters as keys and their respective counts as values.

pub fn letter_count(s: &str) -> HashMap<char, u8> {
    let s = s.to_lowercase();
    let mut char_count = HashMap::new();
    // looping through each char in string and storing in hashmap
    for each_char in s.chars() {
        *char_count.entry(each_char).or_insert(0) += 1
    }
    char_count
}

/// Sorts the vector of tuples alphabetically based on the first element of each tuple.
///
/// This function uses the bubble sort algorithm to sort a vector of tuples alphabetically
/// based on the first element of each tuple.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the vector of tuples to be sorted.
pub fn sort_vecs(arr: &mut Vec<(char, u8)>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j].0 > arr[j + 1].0 {
                arr.swap(j, j + 1);
            }
        }
    }
}