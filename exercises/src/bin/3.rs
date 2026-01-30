use std::{collections::HashMap, io};

fn main() {
    println!("Add employees to the database\nPress enter when done to retrieve");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    'parent: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to readline");
        input = input.trim().to_lowercase();

        if input.is_empty() {
            println!("Retrieving database");
            break;
        }

        let mut chars = input.chars();
        if chars.next() != Some('a')
            || chars.next() != Some('d')
            || chars.next() != Some('d')
            || chars.next() != Some(' ')
        {
            println!("Invalid command start. Try again");
            continue;
        }

        let mut name = String::new();
        loop {
            let c = chars.next();
            if let Some(a) = c {
                if a == ' ' {
                    if name.is_empty() {
                        println!("No name detected. Try again");
                        continue 'parent;
                    }
                    break;
                }
                name.push(a);
            } else {
                println!("Invalid command near name. Try again");
                continue 'parent;
            }
        }

        if chars.next() != Some('t') || chars.next() != Some('o') || chars.next() != Some(' ') {
            println!("Invalid command after name. Try again");
            continue;
        }

        let department: String = chars.collect();

        println!("name: {name}");
        println!("department: {department}");

        if let Some(v) = map.get_mut(&department) {
            v.push(name);
        } else {
            map.insert(department, vec![name]);
        }
    }
    println!("Database: {map:?}");
}
