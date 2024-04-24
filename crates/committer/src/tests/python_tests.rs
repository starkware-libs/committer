use std::collections::HashMap;

pub fn run_test(test_name: String, test_args: HashMap<String, String>) -> String {
    match test_name.as_str() {
        "example_test" => example_test(test_args),
        _ => panic!("Unknown test name: {}", test_name),
    }
}

pub(crate) fn example_test(test_args: HashMap<String, String>) -> String {
    let x = test_args.get("x").expect("Failed to get value for key 'x'");
    let y = test_args.get("y").expect("Failed to get value for key 'y'");

    let output = format!("Calling example test with args: x: {}, y: {}", x, y);
    output
}
