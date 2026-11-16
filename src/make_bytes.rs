use crate::json_builder::{CompleteBranch, CompleteBranches};
use std::fs::read_to_string;

pub fn make_byte() {
    let json = std::fs::read_to_string("Structure.json").expect("kon bestand niet uitlezen");

    let branches: CompleteBranches = serde_json::from_str(&json).expect("kon json niet uitlezen");
    let mut binary_string = String::from("");
    let letters = "appell";

    for letter in letters.chars() {
        for branch in &branches.branches {
            for right_branch in &branch.right_branch {
                if right_branch.key == letter.to_string() {
                    let valueOffunction = herhaal(branch, letter);
                    binary_string.push_str(&*valueOffunction.to_string());
                    continue;
                }

                if right_branch.has_relation == true {
                    let branch_value = branch.value;
                    let binary = right_branch.binary.clone();
                    binary_string.push_str(&*binary.to_string());

                    if let Some(_result_branch) = branches
                        .branches
                        .iter()
                        .find(|branch| branch.value == branch_value)
                    {
                        println!("{}", "Gevonden");
                        println!("{}", _result_branch.value);
                        let valueOffunction = herhaal(_result_branch, letter);
                        binary_string.push_str(&*valueOffunction.to_string())
                    }
                }
            }
        }
    }
    println!("{}", binary_string);
    std::fs::write("codes.txt", binary_string).expect("kon json niet uitlezen");
}

fn herhaal(branch: &CompleteBranch, key: char) -> i32 {
    for branchtaken in &branch.right_branch {
        if branchtaken.key == key.to_string() {
            println!("{}", "Ik zit in de if-statement");
            println!("{}", branchtaken.binary);
            println!("{}", key.to_string());
            let binary = branchtaken.binary.clone();

            return binary;
        }else {
            println!("{}", "Kon niks vinden".to_owned() + &*key.to_string());
        }
    }
    return 0;
}
