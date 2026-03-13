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


    // Exercise 3
    // This exercise code was tweaked a little , The Original code was copied from the Rust community Forum https://users.rust-lang.org/t/3rd-exercise-from-chapter-8-of-the-book/115529
    println!("Welcome to AdminCLI. Do administrative things for a totally real company!");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        // println!("\n\nAvailable commands:\n\t- 'Add <Name> to <Department>' to do exactly that\n\t- 'List <department>` to list every employee in the company in a tree-like structure\n\t- 'List all' to list every employee within this department\n\t- 'Exit' to stop AdminCLI\n");
        println!("{}", HELP_MESSAGE); // we use the constant instead
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
