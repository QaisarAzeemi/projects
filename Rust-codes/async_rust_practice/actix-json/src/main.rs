fn main() {
    println!("Caling serde jason!");
    untyped_example();
    println!("Running Logger!");
    pretty_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");

}

// enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }

use serde_json::{Result, Value};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}