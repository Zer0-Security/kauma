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
            "poly2block" => {
                if let Arguments::Poly2Block { semantic, coefficients } = test_case.arguments {
                    let byte_vect = actions::poly2byte::execute(&semantic, coefficients);
                    json!({"block": actions::de_encode_base64::encode(byte_vect)}) // encoding byte_vect to base 64 String
                } else {
                    json!(null)
                }
            }
            "block2poly" => {
                if let Arguments::Block2Poly { semantic, block } = test_case.arguments {
                    let coefficients = actions::block2poly::execute(&semantic, block);
                    json!({"coefficients": coefficients})
                } else {
                    json!(null)
                }
            }
            "gfmul" => {
                if let Arguments::GfMul { semantic , a, b} = test_case.arguments {
                    let product = actions::gfmul::execute(actions::de_encode_base64::block_to_u128(&semantic, a), actions::de_encode_base64::block_to_u128(&semantic, b));
                    json!({"product": actions::de_encode_base64::encode(product.to_le_bytes())}) // encoding to base 64
                } else {
                    json!(null)
                }
            }
            _ => json!(null), // Fallback for unsupported actions
        };

        // Add the result to the responses map with the ID as the key
        responses.insert(id, result);
    }

    // Prepare and print the final JSON output
    let output = json!({ "responses": responses });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}