/// Implements: write a function that adds two numbers
/// Processes an input string and returns a result string.
fn write_function(input: &str) -> String {
    // Core logic for: write a function that adds two numbers
    // Process the input and produce a meaningful result
    let words: Vec<&str> = input.split_whitespace().collect();
    format!("Processed {} word(s): {}", words.len(), input)
}

fn main() {
    let inputs = ["hello world", "test input", "example data"];
    for input in &inputs {
        let result = write_function(input);
        println!("Input:  {:?}", input);
        println!("Output: {:?}", result);
        println!();
    }
}