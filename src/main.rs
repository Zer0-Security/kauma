mod parser;
mod actions;

use serde_json::json;
use std::env;
use parser::Arguments;
use actions::*;

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
                    let sum = add_numbers::execute(number1, number2);
                    json!({"sum": sum})
                } else {
                    json!(null)
                }
            }
            "subtract_numbers" => {
                if let Arguments::AddSubNumbers { number1, number2 } = test_case.arguments {
                    let difference = subtract_numbers::execute(number1, number2);
                    json!({"difference": difference})
                } else {
                    json!(null)
                }
            }
            "poly2block" => {
                if let Arguments::Poly2Block { semantic, coefficients } = test_case.arguments {
                    let byte_vect = poly2byte::execute(&semantic, coefficients);
                    json!({"block": de_encode_base64::encode(byte_vect)}) // encoding byte_vect to base 64 String
                } else {
                    json!(null)
                }
            }
            "block2poly" => {
                if let Arguments::Block2Poly { semantic, block } = test_case.arguments {
                    let coefficients = block2poly::execute(&semantic, block);
                    json!({"coefficients": coefficients})
                } else {
                    json!(null)
                }
            }
            "gfmul" => {
                if let Arguments::GfMul { semantic , a, b} = test_case.arguments {
                    let product = gfmul::execute(semantic, de_encode_base64::decode( a).unwrap(), de_encode_base64::decode( b).unwrap());
                    json!({"product": de_encode_base64::encode(product)}) // encoding to base 64
                } else {
                    json!(null)
                }
            }
            "sea128" => {
                if let Arguments::Sea128 { mode , key, input} = test_case.arguments {
                    let output = sea128::execute(mode, &de_encode_base64::decode(key).unwrap(), de_encode_base64::decode(input).unwrap());
                    json!({"output": de_encode_base64::encode(output)}) // encoding to base 64
                } else {
                    json!(null)
                }
            }
            "xex" => {
                if let Arguments::Xex { mode , key, tweak, input} = test_case.arguments {
                    let output = xex::execute(mode, key, tweak, input);
                    json!({"output": de_encode_base64::encode(output)}) // encoding to base 64
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