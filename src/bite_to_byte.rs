use crate::json_models::json_byte_map;
use crate::json_models::json_byte_map::BytesMap;

pub fn main() {
    let text = std::fs::read_to_string("src/output/txt/codes.txt").expect("kon bestand niet uitlezen");
    let len = text.len();
    let bits = text;

    let mut bytes: Vec<u8> = Vec::new();
    for chunk in bits.as_bytes().chunks(8) {
        let mut part = String::from_utf8(chunk.to_vec()).unwrap();

        while part.len() < 8 {
            part.push('0');
        }

        let byte = u8::from_str_radix(&part, 2).unwrap();

        bytes.push(byte);
    }

    let byte = json_byte_map::BytesMap {
        bit_length: len,
        bytes,
    };
    let json = serde_json::to_string_pretty(&byte).unwrap();
    std::fs::write("bestand.huff", json)
        .expect("Kon bestand niet schrijven");



    let keys = std::fs::read_to_string("bestand.huff").expect("kon bestand niet uitlezen");
    let group: BytesMap = serde_json::from_str(&keys).expect("kon json niet uitlezen");
    let mut all_bites = String::from("");
    for byte in group.bytes {


         println!("Bits naar byte:");
         println!("{} -> {}", bits, byte);

         let bits_terug = format!("{:08b}", byte);
            all_bites.push_str(&bits_terug);
         println!("Byte naar bits:");
         println!("{} -> {}", byte, bits_terug);
     }
    let lengte = group.bit_length;
    let finish = all_bites.chars().take(lengte).collect::<String>();
    println!("{}", "The end");
    println!("{}", finish);
}