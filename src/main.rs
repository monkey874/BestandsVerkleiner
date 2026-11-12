use std::collections::HashMap;
use std::io::empty;

fn main() {
    let mut letters: Vec<String> = Vec::new();

    let test_text = "appell";
    for letter in test_text.chars() {
        letters.push(letter.to_string());
    }

    let mut frequent_map = HashMap::new();
    for letter in letters.iter() {
        let many_numbers = letters.iter().filter(|&x| x == letter).count();

        frequent_map.insert(letter, many_numbers.to_string());
    }
    let mut frequent_vec: Vec<_> = frequent_map.iter().collect();
    frequent_vec.sort_by_key(|item| item.1);

    let mut values: Vec<i32> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    for (key, value) in frequent_vec.iter() {
        values.push(value.parse::<i32>().unwrap());
        keys.push(key.to_string());
    }

    let numbers_of_values = values.len() - 1;
    let mut result: Vec<i32> = Vec::new();
    for (i, _) in values.iter().enumerate() {
        if i < numbers_of_values {
            if result.is_empty() {
                let first_value = values[i];
                let second_value = values[i + 1];
                println!("{}", first_value + second_value);
                result.push(first_value + second_value);
            }else {
               let third_value = values[i + 1];
               let final_result = third_value + result[0];
                result.remove(0);
                println!("{}", final_result);
                result.push(final_result);
            }
        }
    }
}
