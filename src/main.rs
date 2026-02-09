 use std::collections::HashMap;
 use std::io::{self, Write};

const HELP_MESSAGE: &str = r#"
Available commands:
  - 'Add <Name> to <Department>' to do exactly that
  - 'List <department>` to list every employee in the company in a tree-like structure
  - 'List all' to list every employee within this department
  - 'Exit' to stop AdminCLI
"#;

fn main() {
  // exercises, median and mode from a list of numbers
    println!("First exercise:");
    let mut numbers = vec![100, 200, 32, 498, 81, 32, 67, 71, 50, 74];
    // sort it , then find the median
    println!("numbers: {numbers:?}");
    numbers.sort();
    println!("numbers sorted: {numbers:?}");
    let in_middle = numbers.len() / 2;
    let median = &numbers[in_middle];
    println!("Median: {median}");

    // Find mode
    let mut map = HashMap::new();
    let mut mode: i32 = 0;
    for n in &numbers {
        let count = map.entry(n).or_insert(0);
        *count += 1;
        if *count > mode {
            mode = *n;
        }
    }
    println!("{map:?}");
    println!("Mode: {mode:?}");


    println!("Second exercise:");
    let text = "i want the first apple";
    let vowels = "aeiouAEIOU";
    let mut pig_latin_map = HashMap::new(); // optional map for display
    for word in text.split_whitespace() {
        let first_char = &word[0..1];
        if vowels.contains(first_char) {
            let pig_latin = format!("{word}-hay");
            pig_latin_map.insert(word, pig_latin);
        }else {
            let len_word = word.len();
            let rest = &word[1..len_word];
            let pig_latin = format!("{rest}-{first_char}ay");
            pig_latin_map.insert(word, pig_latin);
        }
    }
    println!("The Text: {text}");
    println!("{pig_latin_map:?}");



    println!("Third exercesize:"); // Tweaked a little , But Original was copied from the Rust community Forum :) https://users.rust-lang.org/t/3rd-exercise-from-chapter-8-of-the-book/115529
    println!("Welcome to AdminCLI. Do administrative things for a totally real company!");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        // println!("\n\nAvailable commands:\n\t- 'Add <Name> to <Department>' to do exactly that\n\t- 'List <department>` to list every employee in the company in a tree-like structure\n\t- 'List all' to list every employee within this department\n\t- 'Exit' to stop AdminCLI\n");
        println!("{}", HELP_MESSAGE); // use the constant instead
        print!("Enter command: ");
        input.clear();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("\nerror: unable to read your input");
        let words: Vec<&str> = input.trim().split(' ').collect();
        match words.as_slice() {
            ["Add", name, "to", dept] =>{
                company
                    .entry(dept.to_string())
                    .or_default()
                    .push(name.to_string());
                println!("Added {} to {}", name, dept);
            }
            ["List", "all"] => {
                if company.is_empty(){
                    println!("No employees yet");
                    continue;
                }
                let mut departments: Vec<&String> = company.keys().collect();
                departments.sort_by_key(|a| a.to_lowercase());
                for dept in departments {
                    println!("\n[{}]", dept);

                    let mut name_refs: Vec<&String> = company.get(dept).unwrap().iter().collect();
                    name_refs.sort_by_key(|a| a.to_lowercase());
                    for name in name_refs {
                        println!("    {}", name);
                    }
                }
            }
            ["List", dept] => match company.get(*dept) {
                Some(names) => {
                    println!("\n[{}]", dept);
                    let mut name_refs: Vec<&String> = names.iter().collect();
                    name_refs.sort_by_key(|a| a.to_lowercase());
                    for name in name_refs {
                        println!("    {}", name);
                    }
                }
                None => {
                    println!("\n'{}' department not found", dept);
                    continue;
                }
            },
            ["Exit"] => {
                println!("\nAdminCLI stopped ... Have a nice day\n");
                break;
            }
            _ => println!("\nunknown command, use only the defined commands"),
        }
    }
    
}
