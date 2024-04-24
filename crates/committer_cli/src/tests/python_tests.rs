use std::collections::HashMap;
use thiserror;

// Define an enum representing different Python tests. with their respective arguments.
pub(crate) enum PythonTest {
    ExampleTest,
    // Add additional test variants as needed
}

/// Error type for PythonTest enum.
#[derive(Debug, thiserror::Error)]
pub(crate) enum PythonTestError {
    #[error("Unknown test name: {0}")]
    UnknownTestName(String),
    #[error("Failed to parse input: {0}")]
    ParseInputError(#[from] serde_json::Error),
}

/// Implements conversion from a string to a `PythonTest`.
impl TryFrom<String> for PythonTest {
    type Error = PythonTestError;

    fn try_from(value: String) -> Result<PythonTest, PythonTestError> {
        match value.as_str() {
            "example_test" => Ok(Self::ExampleTest),
            _ => Err(PythonTestError::UnknownTestName((value).to_string())),
        }
    }
}

impl PythonTest {
    /// Runs the test with the given arguments.
    pub(crate) fn run(&self, input: &str) -> Result<String, PythonTestError> {
        match self {
            Self::ExampleTest => {
                let example_input: HashMap<String, String> = serde_json::from_str(input)?;
                Ok(example_test(example_input))
            }
        }
    }
}

pub(crate) fn example_test(test_args: HashMap<String, String>) -> String {
    let x = test_args.get("x").expect("Failed to get value for key 'x'");
    let y = test_args.get("y").expect("Failed to get value for key 'y'");

    format!("Calling example test with args: x: {}, y: {}", x, y)
}
