pub mod task1;
pub mod task2;
use crate::modules::frequency_task::task2::string_analyzer;

use self::task1::split_string;

// use crate::modules::frequency_task::task2::other;
/// This module provides functions to perform various tasks using the `task1` and `task2` submodules.
/// function to call task 1
pub fn call_task1() {
    split_string();
    // let x=other();
}

/// function to call task 2
pub fn call_task2() {
    let res = string_analyzer();
    println!("{:?}", res);
}
/// merge function to get string with underscore replaced by chars
pub fn merged_task() {
    let mut task1_result = split_string();
    let mut task2_result = string_analyzer();
    // calling replace_string function and storing its result
    let result = replace_string(&mut task1_result, &mut task2_result.letter_count_vec);
    println!("orignal str  : \n \t\t {}", task1_result);
    println!("modified string will be : \n \t\t {}", result);
    println!("modified vec will be : \n \t\t {:?}", task2_result);
}

/// Replaces underscores in a string with characters from a given letter count vector.
///
/// This function takes a string with underscores and a vector containing characters and
/// their counts. It replaces each underscore in the input string with characters from the
/// vector, ensuring that the counts are decremented appropriately.
///
/// # Arguments
///
/// * `input_str` - The input string with underscores to be modified.
/// * `filling_vec` - A mutable reference to a vector containing characters and counts.
///
/// # Returns
///
/// Returns the modified string after replacing underscores.
pub fn replace_string(input_str: &mut str, filling_vec: &mut Vec<(char, u8)>) -> String {
    // iterator through all the chars
    let mut iterator_through_letters = filling_vec.iter_mut();

    // iterating through each char and replacing underscore
    let replaced_result = input_str.chars().map(|c| {
        // if block if char is inderscore
        if c == '_' {
            // replace undescore with char untill its presence in vector
            if let Some(letter_count) = iterator_through_letters.next() {
                if letter_count.1 > 0 {
                    // decrease count of char
                    letter_count.1 -= 1;
                    letter_count.0
                } else {
                    // if count gets zero move to next char in vector
                    return iterator_through_letters.next().map(|l| l.0).unwrap_or('_');
                }
            } else {
                '_'
            }

            //if char is not underscore return the char
        } else {
            c
        }
    });
    // collecting all the chars as string and return it to the function calling it
    let result = replaced_result.collect();
    result
}
