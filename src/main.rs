mod parser;
mod actions;

use serde_json::json;
use std::env;
use parser::Arguments;

fn main() {
    // Collect the path from command-line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Parse the JSON file into test cases
    let test_cases = parser::parse_test_cases(path).expect("Failed to parse JSON");

    // Prepare the response structure
    let mut responses = serde_json::Map::new();

    for (id, test_case) in test_cases {
        let result = match test_case.action.as_str() {
            "add_numbers" => {
                if let Arguments::AddSubNumbers { number1, number2 } = test_case.arguments {
                    let sum = actions::add_numbers::execute(number1, number2);
                    json!({"sum": sum})
                } else {
                    json!(null)
                }
            }
            "subtract_numbers" => {
                if let Arguments::AddSubNumbers { number1, number2 } = test_case.arguments {
                    let difference = actions::subtract_numbers::execute(number1, number2);
                    json!({"difference": difference})
                } else {
                    json!(null)
                }
            }
            _ => json!(null), // Fallback for unsupported actions
        };

        // Add the result to the responses map with the ID as the key
        responses.insert(id, result);
    }

    // Prepare the final JSON output
    let output = json!({ "responses": responses });

    // Print the final JSON result
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}