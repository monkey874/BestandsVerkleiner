use crate::json_models::json_key_references;

pub fn main() {
    let mut text = std::fs::read_to_string("src/output/txt/codes.txt").expect("kon bestand niet uitlezen");
    let keys = std::fs::read_to_string("src/output/json/keys.json").expect("kon bestand niet uitlezen");
    let group: json_key_references::Group = serde_json::from_str(&keys).expect("kon json niet uitlezen");
    let mut newtext = String::from("");
    while !text.is_empty() {
        for x in &group.group {
            let first_value = &x.bites;
            if text.starts_with(&first_value.to_string()) {
                if let Some(_rest) = text.strip_prefix(&first_value.to_string()) {
                    newtext.push(x.key);
                    text = _rest.to_string();
                    break;

                }
            }
        }
    }
    println!("{}", newtext);
}
