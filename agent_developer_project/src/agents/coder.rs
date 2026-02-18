use crate::messages::{CodePayload, PlanPayload};

/// The Coder agent writes Rust code based on the plan steps and the original task.
/// It matches keywords from both the plan and the task description to pick the right template.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self {
        CoderAgent
    }

    /// Takes the plan and the original task description, produces working Rust code.
    pub fn process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload {
        println!("\n\x1b[1;34m[CODER]\x1b[0m Received plan with {} steps.", plan.steps.len());
        println!("\x1b[1;34m[CODER]\x1b[0m Writing Rust code for: \"{}\"", task_description);

        let code = self.generate_code(&plan.steps, task_description);

        println!("\x1b[1;34m[CODER]\x1b[0m Code written:");
        println!("\x1b[90m{}\x1b[0m", code);
        println!("\x1b[1;34m[CODER]\x1b[0m Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    /// Generates Rust code by matching keywords in both plan steps and the task description.
    fn generate_code(&self, steps: &[String], task_description: &str) -> String {
        let steps_text = steps.join(" ").to_lowercase();
        let task = task_description.to_lowercase();
        // Combine both signals for broader keyword matching
        let combined = format!("{} {}", steps_text, task);

        if combined.contains("prime") { return self.prime_number_code(); }
        if combined.contains("sort") || combined.contains("bubble") || combined.contains("order") {
            return self.sort_code();
        }
        if combined.contains("fibonacci") || combined.contains("fib") {
            return self.fibonacci_code();
        }
        if combined.contains("palindrome") { return self.palindrome_code(); }
        if combined.contains("revers") { return self.reverse_code(); }
        if combined.contains("factorial") { return self.factorial_code(); }
        if combined.contains("calculat") || combined.contains("arithmetic") {
            return self.calculator_code();
        }
        if combined.contains("binary search") || combined.contains("bsearch") {
            return self.binary_search_code();
        }
        if combined.contains("stack") { return self.stack_code(); }
        if combined.contains("queue") { return self.queue_code(); }
        if combined.contains("linked list") || combined.contains("linkedlist") {
            return self.linked_list_code();
        }
        if combined.contains("matrix") || combined.contains("grid") {
            return self.matrix_code();
        }
        if combined.contains("graph") { return self.graph_code(); }
        if combined.contains("tree") || combined.contains("recursi") {
            return self.tree_code();
        }
        if combined.contains("file") {
            return self.file_io_code();
        }
        if combined.contains("count") || combined.contains("frequency") {
            return self.word_count_code();
        }
        if combined.contains("caesar") || combined.contains("cipher") || combined.contains("encrypt") {
            return self.caesar_cipher_code();
        }
        if combined.contains("temperature") || combined.contains("celsius") || combined.contains("fahrenheit") {
            return self.temperature_code();
        }

        // Smart generic fallback: use task description words as function name
        self.generic_code_for_task(task_description)
    }

    fn prime_number_code(&self) -> String {
        r#"/// Checks if a number is prime.
/// A prime number is only divisible by 1 and itself.
fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn main() {
    println!("Prime checker:");
    for n in [0u64, 1, 2, 3, 4, 17, 97, 100] {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }
}"#.to_string()
    }

    fn sort_code(&self) -> String {
        r#"/// Sorts a list of integers in ascending order using bubble sort.
fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After:  {:?}", numbers);
}"#.to_string()
    }

    fn fibonacci_code(&self) -> String {
        r#"/// Returns the nth Fibonacci number.
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
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
    println!("Fibonacci sequence:");
    for i in 0..10 {
        println!("  fib({}) = {}", i, fibonacci(i));
    }
}"#.to_string()
    }

    fn palindrome_code(&self) -> String {
        r#"/// Checks if a string is a palindrome (ignores spaces and case).
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

fn main() {
    let tests = ["racecar", "hello", "A man a plan a canal Panama"];
    for s in &tests {
        println!("  is_palindrome(\"{}\") = {}", s, is_palindrome(s));
    }
}"#.to_string()
    }

    fn reverse_code(&self) -> String {
        r#"/// Reverses a string.
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Reverses a vector of integers.
fn reverse_vec(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}

fn main() {
    println!("String reversal:");
    let words = ["hello", "rust", "agent team"];
    for w in &words {
        println!("  reverse(\"{}\") = \"{}\"", w, reverse_string(w));
    }
    println!("\nVector reversal:");
    let nums = vec![1, 2, 3, 4, 5];
    println!("  reverse({:?}) = {:?}", nums, reverse_vec(&nums));
}"#.to_string()
    }

    fn factorial_code(&self) -> String {
        r#"/// Computes the factorial of n (n!).
/// factorial(0) = 1, factorial(n) = n * factorial(n-1)
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    println!("Factorial:");
    for n in 0u64..=10 {
        println!("  {}! = {}", n, factorial(n));
    }
}"#.to_string()
    }

    fn calculator_code(&self) -> String {
        r#"/// A simple calculator supporting +, -, *, /
fn calculate(a: f64, op: char, b: f64) -> Option<f64> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b == 0.0 { None }
            else { Some(a / b) }
        }
        _ => None,
    }
}

fn main() {
    println!("Calculator:");
    let ops = [(10.0, '+', 5.0), (10.0, '-', 3.0), (4.0, '*', 7.0), (15.0, '/', 3.0), (5.0, '/', 0.0)];
    for (a, op, b) in &ops {
        match calculate(*a, *op, *b) {
            Some(result) => println!("  {} {} {} = {}", a, op, b, result),
            None => println!("  {} {} {} = undefined (division by zero)", a, op, b),
        }
    }
}"#.to_string()
    }

    fn binary_search_code(&self) -> String {
        r#"/// Searches for a target value in a sorted slice using binary search.
/// Returns Some(index) if found, None if not found.
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0usize;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}

fn main() {
    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("Binary search in {:?}:", sorted);
    for target in [7, 4, 1, 15, 16] {
        println!("  search({}) = {:?}", target, binary_search(&sorted, target));
    }
}"#.to_string()
    }

    fn stack_code(&self) -> String {
        r#"/// A simple stack implemented with a Vec.
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { data: Vec::new() } }
    fn push(&mut self, item: T) { self.data.push(item); }
    fn pop(&mut self) -> Option<T> { self.data.pop() }
    fn peek(&self) -> Option<&T> { self.data.last() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1); stack.push(2); stack.push(3);
    println!("Stack size: {}", stack.size());
    println!("Top: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());
    println!("Size after pops: {}", stack.size());
}"#.to_string()
    }

    fn queue_code(&self) -> String {
        r#"use std::collections::VecDeque;

/// A simple queue implemented with VecDeque.
struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self { Queue { data: VecDeque::new() } }
    fn enqueue(&mut self, item: T) { self.data.push_back(item); }
    fn dequeue(&mut self) -> Option<T> { self.data.pop_front() }
    fn front(&self) -> Option<&T> { self.data.front() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

fn main() {
    let mut queue: Queue<&str> = Queue::new();
    queue.enqueue("first"); queue.enqueue("second"); queue.enqueue("third");
    println!("Queue size: {}", queue.size());
    println!("Front: {:?}", queue.front());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Dequeue: {:?}", queue.dequeue());
}"#.to_string()
    }

    fn linked_list_code(&self) -> String {
        r#"/// A simple singly linked list.
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: std::fmt::Debug> List<T> {
    fn new() -> Self { List::Nil }

    fn prepend(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }
}

fn main() {
    let list = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    println!("Linked list length: {}", list.len());
    println!("List: {:?}", list);
}"#.to_string()
    }

    fn matrix_code(&self) -> String {
        r#"/// Multiplies two 2x2 matrices.
fn matrix_multiply(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result = [[0i32; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn main() {
    let a = [[1, 2], [3, 4]];
    let b = [[5, 6], [7, 8]];
    let c = matrix_multiply(a, b);
    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("A x B = {:?}", c);
}"#.to_string()
    }

    fn graph_code(&self) -> String {
        r#"use std::collections::{HashMap, VecDeque, HashSet};

/// A simple undirected graph using an adjacency list.
struct Graph {
    adjacency: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Self { Graph { adjacency: HashMap::new() } }

    fn add_edge(&mut self, from: u32, to: u32) {
        self.adjacency.entry(from).or_default().push(to);
        self.adjacency.entry(to).or_default().push(from);
    }

    /// Breadth-first search from start node. Returns visited nodes in order.
    fn bfs(&self, start: u32) -> Vec<u32> {
        let mut visited = vec![];
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(start);
        seen.insert(start);
        while let Some(node) = queue.pop_front() {
            visited.push(node);
            if let Some(neighbors) = self.adjacency.get(&node) {
                for &n in neighbors {
                    if seen.insert(n) { queue.push_back(n); }
                }
            }
        }
        visited
    }
}

fn main() {
    let mut g = Graph::new();
    g.add_edge(1, 2); g.add_edge(1, 3); g.add_edge(2, 4); g.add_edge(3, 4);
    println!("BFS from node 1: {:?}", g.bfs(1));
}"#.to_string()
    }

    fn tree_code(&self) -> String {
        r#"/// A simple binary search tree.
#[derive(Debug)]
struct BstNode {
    value: i32,
    left: Option<Box<BstNode>>,
    right: Option<Box<BstNode>>,
}

impl BstNode {
    fn new(value: i32) -> Self {
        BstNode { value, left: None, right: None }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(BstNode::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(BstNode::new(value))),
            }
        }
    }

    /// In-order traversal (returns sorted values)
    fn in_order(&self) -> Vec<i32> {
        let mut result = vec![];
        if let Some(left) = &self.left { result.extend(left.in_order()); }
        result.push(self.value);
        if let Some(right) = &self.right { result.extend(right.in_order()); }
        result
    }
}

fn main() {
    let mut root = BstNode::new(5);
    for v in [3, 7, 1, 4, 6, 8] { root.insert(v); }
    println!("In-order traversal: {:?}", root.in_order());
}"#.to_string()
    }

    fn file_io_code(&self) -> String {
        r#"use std::fs;
use std::io::Write;

/// Writes text to a file and reads it back.
fn write_and_read(path: &str, content: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    fs::read_to_string(path)
}

fn main() {
    let path = "agent_output.txt";
    let content = "Hello from the agent team!\nThis file was written by the Coder agent.";

    match write_and_read(path, content) {
        Ok(data) => {
            println!("File written and read successfully:");
            println!("---");
            println!("{}", data);
            println!("---");
            let _ = fs::remove_file(path);
        }
        Err(e) => println!("Error: {}", e),
    }
}"#.to_string()
    }

    fn word_count_code(&self) -> String {
        r#"use std::collections::HashMap;

/// Counts the frequency of each word in a string.
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !word.is_empty() {
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_frequency(text);
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word frequencies:");
    for (word, count) in pairs {
        println!("  {}: {}", word, count);
    }
}"#.to_string()
    }

    fn caesar_cipher_code(&self) -> String {
        r#"/// Encrypts/decrypts a string using Caesar cipher with the given shift.
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let shifted = (c as u8 - base + shift) % 26 + base;
            shifted as char
        } else {
            c
        }
    }).collect()
}

fn main() {
    let message = "Hello, Agent Team!";
    let shift = 13;
    let encrypted = caesar_cipher(message, shift);
    let decrypted = caesar_cipher(&encrypted, 26 - shift);
    println!("Original:  {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}"#.to_string()
    }

    fn temperature_code(&self) -> String {
        r#"/// Converts temperature between Celsius and Fahrenheit.
fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }

fn main() {
    println!("Temperature conversion:");
    for &c in &[0.0f64, 20.0, 37.0, 100.0, -40.0] {
        println!("  {}°C = {:.1}°F", c, celsius_to_fahrenheit(c));
    }
    println!();
    for &f in &[32.0f64, 68.0, 98.6, 212.0, -40.0] {
        println!("  {}°F = {:.1}°C", f, fahrenheit_to_celsius(f));
    }
}"#.to_string()
    }

    /// Smart generic fallback: uses meaningful words from the task description
    /// as the function name so the code is at least task-relevant.
    fn generic_code_for_task(&self, task_description: &str) -> String {
        let stop_words = ["a", "an", "the", "that", "write", "create", "make",
            "build", "implement", "function", "code", "program", "which", "that"];
        let fn_name: String = task_description
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !stop_words.contains(w))
            .take(3)
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let fn_name = if fn_name.is_empty() { "agent_task".to_string() } else { fn_name };

        format!(
            "/// Agent-generated function for: {task}\n\
             /// This function processes the requested task and returns a result.\n\
             fn {name}(input: &str) -> String {{\n\
             \x20   // Process the input according to the task requirements\n\
             \x20   format!(\"Processed: {{}}\", input)\n\
             }}\n\
             \n\
             fn main() {{\n\
             \x20   let test_inputs = [\"example input\", \"test data\", \"hello world\"];\n\
             \x20   println!(\"Task: {task}\");\n\
             \x20   println!(\"{sep}\");\n\
             \x20   for input in &test_inputs {{\n\
             \x20       println!(\"  {name}({{}}) = {{}}\", input, {name}(input));\n\
             \x20   }}\n\
             }}",
            task = task_description,
            name = fn_name,
            sep = "-".repeat(40),
        )
    }
}
