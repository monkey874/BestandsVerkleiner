use crate::json_builder::{CompleteBranch, CompleteBranches};
use crate::jsonbuilder2;
use crate::jsonbuilder2::{CompleteGroup};

pub fn make_byte() {
    let json = std::fs::read_to_string("Structure.json").expect("kon bestand niet uitlezen");
    let mut complete_group = jsonbuilder2::Group {
        group: Vec::new(),
    };
    let branches: CompleteBranches = serde_json::from_str(&json).expect("kon json niet uitlezen");
    let mut binary = String::new();

    for letter in "appeltaart".chars() {

        if let Some(code) =
            find_code(
                &branches,
                &branches.branches[0],
                &letter.to_string(),
                String::new(),
            )
        {
            binary.push_str(&code);
            let group = CompleteGroup {
                key: letter,
                bytes: code,
            };

            complete_group.group.push(group);

        }
    }

    let json = serde_json::to_string_pretty(&complete_group).unwrap();
    std::fs::write("keys.json", json).expect("Unable to write file");

    println!("{}", binary);
    std::fs::write("codes.txt", binary).expect("Unable to write file");
}
fn find_code(
    branches: &CompleteBranches,
    branch: &CompleteBranch,
    target: &str,
    path: String,
) -> Option<String> {

    for left in &branch.left_branch {
        if left.key == target {
            return Some(format!("{}0", path));
        }
    }

    for right in &branch.right_branch {
        if right.key == target {
            return Some(format!("{}1", path));
        }
    }

    for right in &branch.right_branch {
        if right.has_relation {

            if let Some(next_branch) = branches
                .branches
                .iter()
                .find(|b| b.value == right.value)
            {
                if let Some(code) = find_code(
                    branches,
                    next_branch,
                    target,
                    format!("{}1", path),
                ) {
                    return Some(code);
                }
            }
        }
    }

    None
}