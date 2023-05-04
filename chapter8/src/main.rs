use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut u = vec![1, 2, 3];
    let mut w = Vec::new();
    w.push(5);
    w.push(8);

    let third: &i32 = &u[2];
    println!("the third element of u is {third}");

    let third: Option<&i32> = u.get(2);
    match third {
        Some(third) => println!("the third element of u is {third}"),
        None => println!("No third element in u"),
    }
    for i in &mut u {
        *i += 12;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(5.2),
        SpreadsheetCell::Text(String::from("Blja!")),
    ];

    let data = "negrler";
    let mut s = data.to_string();
    let mut t = String::new();
    s.push_str(" umrite");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blau"), 10);
    scores.insert(String::from("Rot"), 50);
    scores.entry(String::from("Gelb")).or_insert(0);

    let team_name = String::from("Blau");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hallo hallo herr akha und und aruna";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    //ex 1

    let numbers = vec![1,2,2,3,4,5,5,5];
    let (median, mode) = median_and_mode(&numbers);
    println!("Median: {} und Mode: {}", median, mode);

    //ex 2

    //ex 3

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("Enter a command");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input = "Exit" {
            break;
        }
        else {
            let words: Vec<&str> = input.split_whitespace().collect();
            if words.len() < 4 || words[0].to_lowercase() != "add" || words[2].to_lowercase() != "to" {
                println!("Incorrect command type. Use the format: Add [Name] to [Department]");
            }

            let employee = words[1].to_string();
            let department = words[3].to_string();

            company.entry(department).copied().
            println!("Added {} to {} department.", employee, department);
        }
    }
    for (department, employees) in &mut company {
        employees.sort();
        println!("\n{} department employees:", department);
        for employee in employees {
            println!("{}", employee);
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

//exercises

fn median_and_mode(numbers: &[i32]) -> (f32, i32) {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();

    let median = if sorted_numbers.len() %2 == 0 {
        (sorted_numbers[sorted_numbers.len()/2 -1]) as f32 + sorted_numbers[sorted_numbers.len()/2] as f32) /2.0
    } else {
        sorted_numbers[sorted_numbers.len()/2] as f32
    };

    let mut occurrences = HashMap::new();
    for number in numbers {
        occurrences.entry(number).or_insert(0);
        occurrences += 1;
    }

    let mode = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
        .unwrap();
    
        (median, mode)
}

fn pig_latin(s: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    s.split_whitespace()
        .map(|word| {
            let first_char = word.chars().nth(0).unwrap();
            if vowels.contains(&first_char) {
                format!("{}-hay", word)
            } else {
                format!("{}{}-ay", &word[1..], first_char)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}