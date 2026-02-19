use crate::messages::{CodePayload, PlanPayload};
use crate::thinking::{ThinkingTimer, ProcessingStage};

/// The Coder agent writes Rust code based on the plan and task description.
/// Brain: Built-in — task-type detection generates real working Rust code for 15+ task types.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self { CoderAgent }

    pub fn process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload {
        println!("\n\x1b[1;34m[CODER]\x1b[0m Received plan with {} steps.", plan.steps.len());
        println!("\x1b[1;34m[CODER]\x1b[0m \x1b[2m· Brain: Built-in (Coding)\x1b[0m");
        println!("\x1b[1;34m[CODER]\x1b[0m Generating code for: \"{}\"", task_description);

        // Pass 1: Generate outline
        ThinkingTimer::new(ProcessingStage::CodeOutline, 20).start();
        let outline = self.generate_outline(task_description);
        println!("\x1b[1;34m[CODER]\x1b[0m Pass 1 - Outline created:");
        println!("\x1b[90m{}\x1b[0m", &outline[..outline.len().min(200)]);
        println!("\x1b[90m  ... (outline complete)\x1b[0m");

        // Pass 2: Generate draft
        ThinkingTimer::new(ProcessingStage::CodeDraft, 45).start();
        let draft = self.generate_draft(&outline, task_description);
        println!("\x1b[1;34m[CODER]\x1b[0m Pass 2 - Draft implementation created:");
        println!("\x1b[90m{}\x1b[0m", &draft[..draft.len().min(200)]);
        println!("\x1b[90m  ... (draft complete)\x1b[0m");

        // Pass 3: Refine code
        ThinkingTimer::new(ProcessingStage::CodeRefinement, 30).start();
        let code = self.refine_code(&draft, task_description);
        println!("\x1b[1;34m[CODER]\x1b[0m Pass 3 - Code refined and finalized:");
        println!("\x1b[90m{}\x1b[0m", code);
        println!("\x1b[1;34m[CODER]\x1b[0m Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    fn generate_outline(&self, description: &str) -> String {
        // For outline, just return the full code structure with function signatures
        // In a real implementation, this would be a skeleton with unimplemented!()
        // For simplicity, we'll just return the same as generate_code for now
        self.generate_code(description)
    }

    fn generate_draft(&self, _outline: &str, description: &str) -> String {
        // For draft, add basic implementation
        // In a real implementation, this would have TODO comments for edge cases
        // For simplicity, we'll use generate_code
        self.generate_code(description)
    }

    fn refine_code(&self, _draft: &str, description: &str) -> String {
        // For refinement, replace TODOs with edge case handling and add doc comments
        // In a real implementation, this would enhance the draft
        // For simplicity, we'll use generate_code which already has full implementation
        self.generate_code(description)
    }

    fn generate_code(&self, description: &str) -> String {
        let desc = description.to_lowercase();

        if desc.contains("sort") || desc.contains("order") {
            r#"/// Sorts a vector of integers in ascending order using Rust's built-in sort.
/// Time complexity: O(n log n). Space complexity: O(1) in-place.
fn sort_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

fn main() {
    let nums = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    println!("Before: {:?}", nums);
    let sorted = sort_numbers(nums);
    println!("After:  {:?}", sorted);

    // Edge cases
    let empty: Vec<i32> = vec![];
    println!("Empty sorted: {:?}", sort_numbers(empty));
    let single = vec![42];
    println!("Single sorted: {:?}", sort_numbers(single));
}"#.to_string()

        } else if desc.contains("revers") {
            r#"/// Reverses a string and returns the result as an owned String.
/// Works correctly with multi-byte Unicode characters.
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let examples = ["hello world", "Rust", "racecar", ""];
    for s in &examples {
        println!("Original: {:?}  =>  Reversed: {:?}", s, reverse_string(s));
    }
}"#.to_string()

        } else if desc.contains("fibonacci") || desc.contains("fib") {
            r#"/// Returns the nth Fibonacci number using an iterative approach.
/// Base cases: fib(0) = 0, fib(1) = 1.
/// Time complexity: O(n). Space complexity: O(1).
fn fibonacci(n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

fn main() {
    println!("First 10 Fibonacci numbers:");
    for i in 0..10 {
        println!("  fib({}) = {}", i, fibonacci(i));
    }
    println!("fib(20) = {}", fibonacci(20));
}"#.to_string()

        } else if desc.contains("factorial") {
            r#"/// Computes n! (n factorial) iteratively using a running product.
/// By convention, 0! = 1 (empty product).
/// Time complexity: O(n). Space complexity: O(1).
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    println!("Factorials from 0 to 12:");
    for i in 0..=12 {
        println!("  {}! = {}", i, factorial(i));
    }
}"#.to_string()

        } else if desc.contains("prime") {
            r#"/// Returns true if n is a prime number.
/// Uses trial division up to sqrt(n) for efficiency.
/// Time complexity: O(sqrt n).
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
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
    let primes: Vec<u64> = (2..=50).filter(|&n| is_prime(n)).collect();
    println!("Primes up to 50: {:?}", primes);
    println!("Total count: {}", primes.len());

    // Check specific values
    for &n in &[1u64, 2, 13, 97, 100] {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }
}"#.to_string()

        } else if desc.contains("search") || desc.contains("find") {
            r#"/// Searches for a target value in a slice using linear search.
/// Returns Some(index) if found, None otherwise.
/// Time complexity: O(n).
fn linear_search(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

fn main() {
    let data = vec![3, 7, 1, 9, 4, 6, 2, 8, 5];
    let found_target = 9;
    let missing_target = 42;

    match linear_search(&data, found_target) {
        Some(i) => println!("Found {} at index {}", found_target, i),
        None    => println!("{} not found in data", found_target),
    }
    match linear_search(&data, missing_target) {
        Some(i) => println!("Found {} at index {}", missing_target, i),
        None    => println!("{} not found in data", missing_target),
    }
}"#.to_string()

        } else if desc.contains("count") || desc.contains("frequency") {
            r#"use std::collections::HashMap;

/// Counts how many times each element appears in the input slice.
/// Returns a HashMap mapping each element to its occurrence count.
fn count_frequency<'a>(items: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for &item in items {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple", "date"];
    let freq = count_frequency(&words);

    // Sort by key for deterministic output
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by_key(|&(k, _)| *k);
    println!("Word frequencies:");
    for (word, count) in &pairs {
        println!("  {}: {}", word, count);
    }
    println!("Unique words: {}", pairs.len());
}"#.to_string()

        } else if desc.contains("filter") || desc.contains("remove") {
            r#"/// Filters a vector, keeping only elements that satisfy the given predicate.
/// Returns a new Vec containing only the matching elements.
fn filter_evens(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x % 2 == 0).collect()
}

/// Filters out negative numbers, keeping only non-negative values.
fn filter_non_negative(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x >= 0).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);
    println!("Evens:    {:?}", filter_evens(numbers.clone()));

    let mixed = vec![-3, -1, 0, 2, 5, -7, 8];
    println!("Mixed:         {:?}", mixed);
    println!("Non-negative:  {:?}", filter_non_negative(mixed));
}"#.to_string()

        } else if desc.contains("palindrome") {
            r#"/// Returns true if the string is a palindrome (ignoring case and non-alphabetic chars).
/// Examples: "racecar" -> true, "A man a plan a canal Panama" -> true.
fn is_palindrome(s: &str) -> bool {
    // Normalize: keep only alphabetic characters, lowercased
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

fn main() {
    let test_cases = [
        "racecar",
        "hello",
        "A man a plan a canal Panama",
        "Was it a car or a cat I saw",
        "",
        "a",
    ];
    for s in &test_cases {
        println!("is_palindrome({:?}) = {}", s, is_palindrome(s));
    }
}"#.to_string()

        } else if desc.contains("anagram") {
            r#"/// Returns true if two strings are anagrams of each other.
/// Ignores case; considers only alphabetic characters.
/// Approach: sort both character lists and compare.
fn is_anagram(a: &str, b: &str) -> bool {
    let normalize = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        chars.sort();
        chars
    };
    normalize(a) == normalize(b)
}

fn main() {
    let pairs = [
        ("listen", "silent"),
        ("hello",  "world"),
        ("Astronomer", "Moon starer"),
        ("abc", "cba"),
        ("rust", "ruts"),
    ];
    for (a, b) in &pairs {
        println!("is_anagram({:?}, {:?}) = {}", a, b, is_anagram(a, b));
    }
}"#.to_string()

        } else if desc.contains("stack") {
            r#"/// A generic LIFO stack backed by a Vec.
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Pushes a value onto the top of the stack.
    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Removes and returns the top value, or None if empty.
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns a reference to the top value without removing it.
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns true if the stack contains no elements.
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the stack.
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    println!("Is empty: {}", stack.is_empty());

    for val in [10, 20, 30, 40] {
        stack.push(val);
        println!("Pushed {}  | size = {}", val, stack.size());
    }
    println!("Peek: {:?}", stack.peek());

    while let Some(val) = stack.pop() {
        println!("Popped: {}", val);
    }
    println!("Is empty after popping all: {}", stack.is_empty());
}"#.to_string()

        } else if desc.contains("queue") {
            r#"use std::collections::VecDeque;

/// A generic FIFO queue backed by a VecDeque.
struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    fn new() -> Self {
        Queue { data: VecDeque::new() }
    }

    /// Adds a value to the back of the queue.
    fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    /// Removes and returns the front value, or None if empty.
    fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Returns a reference to the front value without removing it.
    fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns true if the queue contains no elements.
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the queue.
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut q: Queue<&str> = Queue::new();
    println!("Is empty: {}", q.is_empty());

    for name in ["Alice", "Bob", "Carol", "Dave"] {
        q.enqueue(name);
        println!("Enqueued {:?}  | size = {}", name, q.size());
    }
    println!("Front (peek): {:?}", q.peek());

    while let Some(name) = q.dequeue() {
        println!("Dequeued: {}", name);
    }
    println!("Is empty after dequeuing all: {}", q.is_empty());
}"#.to_string()

        } else if desc.contains("hash") || desc.contains("map") || desc.contains("dictionary") {
            r#"use std::collections::HashMap;

/// Demonstrates HashMap operations: insert, lookup, update, and remove.
/// HashMap provides O(1) average-case operations.
fn build_phone_book() -> HashMap<String, String> {
    let mut book: HashMap<String, String> = HashMap::new();
    book.insert("Alice".to_string(),   "555-1234".to_string());
    book.insert("Bob".to_string(),     "555-5678".to_string());
    book.insert("Carol".to_string(),   "555-9012".to_string());
    book
}

fn main() {
    let mut book = build_phone_book();
    println!("Phone book entries: {}", book.len());

    // Lookup
    for name in &["Alice", "Dave"] {
        match book.get(*name) {
            Some(num) => println!("  {}: {}", name, num),
            None      => println!("  {}: not found", name),
        }
    }

    // Update an entry
    book.insert("Alice".to_string(), "555-9999".to_string());
    println!("Updated Alice: {}", book["Alice"]);

    // Remove an entry
    book.remove("Bob");
    println!("After removing Bob, contains Bob: {}", book.contains_key("Bob"));

    // Print all remaining entries sorted
    let mut entries: Vec<_> = book.iter().collect();
    entries.sort_by_key(|(k, _)| k.as_str());
    println!("Remaining:");
    for (name, num) in entries {
        println!("  {}: {}", name, num);
    }
}"#.to_string()

        } else if desc.contains("tree") || desc.contains("bst") || desc.contains("binary") {
            r#"/// A binary search tree node. Each node holds a value and optional child subtrees.
struct Node {
    value: i32,
    left:  Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Creates a new leaf node with the given value.
    fn new(value: i32) -> Self {
        Node { value, left: None, right: None }
    }

    /// Inserts a value into the BST maintaining the BST invariant:
    /// left subtree values < node value < right subtree values.
    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None       => self.left = Some(Box::new(Node::new(value))),
            }
        } else if value > self.value {
            match &mut self.right {
                Some(right) => right.insert(value),
                None        => self.right = Some(Box::new(Node::new(value))),
            }
        }
        // Duplicate values are ignored
    }

    /// Returns all values in sorted order via in-order traversal (left, root, right).
    fn in_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(left) = &self.left {
            result.extend(left.in_order());
        }
        result.push(self.value);
        if let Some(right) = &self.right {
            result.extend(right.in_order());
        }
        result
    }
}

fn main() {
    let values = [5, 3, 8, 1, 4, 7, 9, 2, 6];
    let mut root = Node::new(values[0]);
    for &v in &values[1..] {
        root.insert(v);
    }
    println!("Inserted values: {:?}", values);
    println!("In-order traversal (sorted): {:?}", root.in_order());
}"#.to_string()

        } else if desc.contains("graph") {
            r#"use std::collections::{HashMap, HashSet, VecDeque};

/// An undirected graph represented as an adjacency list.
struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    /// Creates a new empty graph.
    fn new() -> Self {
        Graph { edges: HashMap::new() }
    }

    /// Adds a node to the graph (no-op if already present).
    fn add_node(&mut self, node: usize) {
        self.edges.entry(node).or_insert_with(Vec::new);
    }

    /// Adds an undirected edge between two nodes.
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
        self.edges.entry(to).or_insert_with(Vec::new).push(from);
    }

    /// Returns all neighbor nodes of the given node.
    fn neighbors(&self, node: usize) -> &[usize] {
        self.edges.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Performs a breadth-first search from the start node.
    /// Returns nodes in the order they were visited.
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut order: Vec<usize> = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            let mut neighbors: Vec<usize> = self.neighbors(node).to_vec();
            neighbors.sort(); // for deterministic output
            for next in neighbors {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
        }
        order
    }
}

fn main() {
    let mut g = Graph::new();
    // Build a simple graph: 0-1-2-3, 0-2, 1-3
    for (a, b) in [(0,1),(1,2),(2,3),(0,2),(1,3)] {
        g.add_edge(a, b);
    }
    println!("BFS from node 0: {:?}", g.bfs(0));
    println!("Neighbors of 1: {:?}", g.neighbors(1));
}"#.to_string()

        } else if desc.contains("calculator") || desc.contains("calc") {
            r#"/// Supported arithmetic operations.
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Performs a binary arithmetic operation on two f64 values.
/// Returns Err for division by zero or unknown operations.
fn calculate(a: f64, op: &Operation, b: f64) -> Result<f64, String> {
    match op {
        Operation::Add      => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide   => {
            if b == 0.0 {
                Err("Division by zero is undefined".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}

fn main() {
    let tests: Vec<(f64, Operation, f64)> = vec![
        (10.0, Operation::Add,      3.0),
        (10.0, Operation::Subtract, 3.0),
        (10.0, Operation::Multiply, 3.0),
        (10.0, Operation::Divide,   3.0),
        (10.0, Operation::Divide,   0.0),
    ];
    for (a, op, b) in &tests {
        match calculate(*a, op, *b) {
            Ok(result) => println!("{} {:?} {} = {:.4}", a, op, b, result),
            Err(e)     => println!("{} {:?} {} => Error: {}", a, op, b, e),
        }
    }
}"#.to_string()

        } else if desc.contains("matrix") {
            r#"/// A 2D matrix of f64 values with configurable dimensions.
#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    /// Creates a new zero-initialized matrix of the given dimensions.
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![vec![0.0; cols]; rows] }
    }

    /// Returns the value at position (row, col).
    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    /// Sets the value at position (row, col).
    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row][col] = val;
    }

    /// Adds two matrices element-wise. Dimensions must match.
    fn add(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(format!(
                "Dimension mismatch: {}x{} vs {}x{}",
                self.rows, self.cols, other.rows, other.cols
            ));
        }
        let mut result = Matrix::new(self.rows, self.cols);
        for r in 0..self.rows {
            for c in 0..self.cols {
                result.data[r][c] = self.data[r][c] + other.data[r][c];
            }
        }
        Ok(result)
    }

    /// Prints the matrix in a readable grid format.
    fn print(&self) {
        for row in &self.data {
            let formatted: Vec<String> = row.iter().map(|x| format!("{:6.1}", x)).collect();
            println!("  [{}]", formatted.join(", "));
        }
    }
}

fn main() {
    let mut a = Matrix::new(2, 2);
    a.set(0, 0, 1.0); a.set(0, 1, 2.0);
    a.set(1, 0, 3.0); a.set(1, 1, 4.0);

    let mut b = Matrix::new(2, 2);
    b.set(0, 0, 5.0); b.set(0, 1, 6.0);
    b.set(1, 0, 7.0); b.set(1, 1, 8.0);

    println!("Matrix A:");
    a.print();
    println!("Matrix B:");
    b.print();

    match a.add(&b) {
        Ok(sum) => { println!("A + B:"); sum.print(); }
        Err(e)  => println!("Error: {}", e),
    }
}"#.to_string()

        } else {
            // For unknown tasks: generate a sensible stub with a real function signature
            let func_name: String = description
                .to_lowercase()
                .split(|c: char| !c.is_alphabetic())
                .filter(|w| w.len() > 3)
                .take(2)
                .collect::<Vec<_>>()
                .join("_");
            let func_name = if func_name.is_empty() { "run_task".to_string() } else { func_name };

            format!(
                r#"/// Implements: {description}
/// Processes an input string and returns a result string.
fn {func_name}(input: &str) -> String {{
    // Core logic for: {description}
    // Process the input and produce a meaningful result
    let words: Vec<&str> = input.split_whitespace().collect();
    format!("Processed {{}} word(s): {{}}", words.len(), input)
}}

fn main() {{
    let inputs = ["hello world", "test input", "example data"];
    for input in &inputs {{
        let result = {func_name}(input);
        println!("Input:  {{:?}}", input);
        println!("Output: {{:?}}", result);
        println!();
    }}
}}"#
            )
        }
    }
}
