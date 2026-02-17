use crate::messages::{CodePayload, PlanPayload};

/// The Coder agent writes Rust code based on the plan steps.
/// Think of this as a developer implementing what the planner designed.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self {
        CoderAgent
    }

    /// Takes the plan and produces working Rust code.
    pub fn process(&self, plan: PlanPayload) -> CodePayload {
        println!("\n[CODER] Received plan with {} steps.", plan.steps.len());
        println!("[CODER] Writing Rust code...");

        let code = self.generate_code(&plan.steps);

        println!("[CODER] Code written. Here's what I produced:");
        println!("---");
        println!("{}", code);
        println!("---");
        println!("[CODER] Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    /// Generates Rust code based on plan step content.
    /// Keyword matching on steps determines what kind of code to write.
    fn generate_code(&self, steps: &[String]) -> String {
        let steps_text = steps.join(" ").to_lowercase();

        if steps_text.contains("prime") {
            return self.prime_number_code();
        }
        if steps_text.contains("sort") {
            return self.sort_code();
        }
        if steps_text.contains("fibonacci") {
            return self.fibonacci_code();
        }
        if steps_text.contains("palindrome") || steps_text.contains("string") {
            return self.palindrome_code();
        }

        // Default: generic function template
        self.generic_code()
    }

    fn prime_number_code(&self) -> String {
        r#"/// Checks if a number is prime.
/// A prime number is only divisible by 1 and itself.
/// Numbers less than or equal to 1 are not prime.
fn is_prime(n: u64) -> bool {
    // Edge case: 0 and 1 are not prime
    if n <= 1 {
        return false;
    }
    // 2 is the only even prime
    if n == 2 {
        return true;
    }
    // All other even numbers are not prime
    if n % 2 == 0 {
        return false;
    }
    // Check odd divisors up to the square root
    // (If n has a factor larger than sqrt(n), the other factor must be smaller)
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    // Test a range of numbers
    let test_cases = [0, 1, 2, 3, 4, 17, 18, 97, 100];
    println!("Prime number checker:");
    println!("---------------------");
    for &n in &test_cases {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }
}"#
        .to_string()
    }

    fn sort_code(&self) -> String {
        r#"/// Sorts a list of integers in ascending order using bubble sort.
/// Bubble sort repeatedly swaps adjacent elements that are out of order.
fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    // Repeat for each element
    for i in 0..n {
        // Each pass moves the largest unsorted element to its correct position
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before sort: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After sort:  {:?}", numbers);
}"#
        .to_string()
    }

    fn fibonacci_code(&self) -> String {
        r#"/// Returns the nth Fibonacci number.
/// The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, ...
/// Each number is the sum of the two before it.
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        // For n >= 2, sum the previous two values iteratively
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

fn main() {
    println!("Fibonacci sequence (first 10 terms):");
    for i in 0..10 {
        println!("  fibonacci({}) = {}", i, fibonacci(i));
    }
}"#
        .to_string()
    }

    fn palindrome_code(&self) -> String {
        r#"/// Checks if a string is a palindrome.
/// A palindrome reads the same forwards and backwards (e.g., "racecar").
/// This version ignores spaces and is case-insensitive.
fn is_palindrome(s: &str) -> bool {
    // Normalize: lowercase and remove spaces
    let cleaned: String = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    // Compare the string to its reverse
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

fn main() {
    let test_cases = ["racecar", "hello", "A man a plan a canal Panama", "rust"];
    println!("Palindrome checker:");
    println!("-------------------");
    for &s in &test_cases {
        println!("  is_palindrome(\"{}\") = {}", s, is_palindrome(s));
    }
}"#
        .to_string()
    }

    fn generic_code(&self) -> String {
        r#"/// A general-purpose function generated by the agent.
/// Processes an input value and returns a result.
fn process(input: i32) -> i32 {
    // Apply a transformation to the input
    input * 2 + 1
}

fn main() {
    println!("Agent-generated function output:");
    println!("--------------------------------");
    for i in 0..5 {
        println!("  process({}) = {}", i, process(i));
    }
}"#
        .to_string()
    }
}
