mod json_builder;
pub mod make_bytes;
pub mod jsonbuilder2;

use std::collections::HashMap;
use std::fs;
use crate::json_builder::{CompleteBranch, CompleteBranches, RightBranch};

fn main() {

    let mut letters: Vec<String> = Vec::new();

    let test_text = "";
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
    let mut complete_branches = CompleteBranches {
        branches: Vec::new(),
    };

    let mut used_keys: Vec<String> = Vec::new();
    for (i, _) in values.iter().enumerate() {
        if i < numbers_of_values {
            if result.is_empty() {

                let first_value = values[i];
                let second_value = values[i + 1];
                let info_left_branch = json_builder::LeftBranch {
                    value: first_value,
                    key: get_key_or_none(&*keys[i], &mut used_keys),
                    has_relation: false,
                    binary: 0
                };
                used_keys.push(keys[i].to_string());
                let info_right_branch = RightBranch {
                    value: second_value,
                    key: get_key_or_none(&*keys[i + 1], &mut used_keys),
                    has_relation: false,
                    binary: 1
                };

                used_keys.push(keys[i + 1].to_string());
                let complete_branch = CompleteBranch {
                    value: first_value + second_value,
                    right_branch: vec![info_right_branch],
                    left_branch: vec![info_left_branch],
                };

                complete_branches.branches.push(complete_branch);

                result.push(first_value + second_value);
            }else {
               let third_value = values[i + 1];
               let final_result = third_value + result[0];


                let info_right_branch = RightBranch {
                    value: result[0],
                    key: get_key_or_none(&*keys[i], &mut used_keys),
                    has_relation: true,
                    binary: 1

                };

                used_keys.push(keys[i].to_string());
                let info_left_branch = json_builder::LeftBranch {
                    value: third_value,
                    key: get_key_or_none(&*keys[i + 1], &mut used_keys),
                    has_relation: false,
                    binary: 0
                };
                used_keys.push(keys[i + 1].to_string());
                let complete_branch = CompleteBranch {
                    value: result[0] + third_value,
                    right_branch: vec![info_right_branch],
                    left_branch: vec![info_left_branch],
                };

                complete_branches.branches.push(complete_branch);


                result.remove(0);
                result.push(final_result);
            }

        }

    }
    complete_branches.branches.reverse();
    let json = serde_json::to_string_pretty(&complete_branches).unwrap();
    let file_name = "Structure.json".to_string();
    std::fs::write(file_name, json).expect("TODO: panic message");

    make_bytes::make_byte();
}
fn get_key_or_none(key: &str, used_keys: &mut Vec<String>) -> String {
    if used_keys.contains(&key.to_string()) {
        "none".to_string()
    } else {
        used_keys.push(key.to_string());
        key.to_string()
    }
}


