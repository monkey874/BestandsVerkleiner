mod json_builder;
use std::collections::HashMap;

use crate::json_builder::{CompleteBranch, RightBranch};

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
                let info_left_branch = json_builder::LeftBranch {
                    value: first_value,
                    key: keys[i].to_string(),
                    has_relation: false,
                    binary: 0
                };
                let info_right_branch = RightBranch {
                    value: second_value,
                    key: keys[i + 1].to_string(),
                    has_relation: false,
                    binary: 1
                };
                let complete_branch = CompleteBranch {
                    value: first_value + second_value,
                    right_branch: vec![info_right_branch],
                    left_branch: vec![info_left_branch],
                };

                let json = serde_json::to_string_pretty(&complete_branch);
                let file_name = format!("Structure {}.json", i);
                std::fs::write(file_name, json.unwrap_or(0.to_string())).expect("TODO: panic message");

                result.push(first_value + second_value);
            }else {
               let third_value = values[i + 1];
               let final_result = third_value + result[0];


                let info_right_branch = RightBranch {
                    value: result[0],
                    key: keys[i].to_string(),
                    has_relation: true,
                    binary: 1

                };
                let info_left_branch = json_builder::LeftBranch {
                    value: third_value,
                    key: keys[i + 1].to_string(),
                    has_relation: false,
                    binary: 0
                };
                let complete_branch = CompleteBranch {
                    value: result[0] + third_value,
                    right_branch: vec![info_right_branch],
                    left_branch: vec![info_left_branch],
                };

                let json = serde_json::to_string_pretty(&complete_branch);
                let file_name = format!("Structure {}.json", i);
                std::fs::write(file_name, json.unwrap_or(2.to_string())).expect("TODO: panic message");

                result.remove(0);
                result.push(final_result);
            }
        }

    }

}
