use crate::jsonbuilder2::Group;
use crate::jsonbuilder3;
use crate::jsonbuilder3::HuffFile;

pub fn main() {
    let mut text = std::fs::read_to_string("codes.txt").expect("kon bestand niet uitlezen");
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

    let byte = jsonbuilder3::HuffFile {
        bit_length: len,
        data: bytes,
    };
    let json = serde_json::to_string_pretty(&byte).unwrap();
    std::fs::write("bestand.huff", json)
        .expect("Kon bestand niet schrijven");



    let keys = std::fs::read_to_string("bestand.huff").expect("kon bestand niet uitlezen");
    let group: HuffFile = serde_json::from_str(&keys).expect("kon json niet uitlezen");
    let mut all_bites = String::from("");
    for byte in group.data {


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