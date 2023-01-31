//use serde_json::{Number, Value};
/*
pub fn readAFile(filename: String) {


}*/

pub fn read_from_string(text: String, output_path: String) {
    let json: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");

    std::fs::write(output_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();

    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
