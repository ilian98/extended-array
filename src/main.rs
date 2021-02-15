mod exray;
mod functions;
use exray::*;
use functions::*;

use std::{fs::File, io::Error};
use std::io::{self, BufRead, BufReader, Lines, StdinLock};

#[derive(Debug)]
pub enum CommandError {
    InputEnd(String),
    IOError(io::Error),
    FileError(io::Error),
    CreateError(String),
}

fn try_line (result: Option<Result<String, Error>>) -> Result<Vec<String>, CommandError> {
    match result {
        None => return Err(CommandError::InputEnd(String::from("Input ended without reading the numbers"))),
        Some(line) => {
            if line.is_err() {
                return Err(CommandError::IOError(line.unwrap_err()));
            }
            
            return Ok(line.as_ref().unwrap().trim().split(',').map(|s| s.to_string()).collect());
        }
    }
}

use std::collections::{HashSet, HashMap};
type FuncMap = HashMap::<String, Func<i64>>;

fn create (words: Vec<String>, line_it: &mut Lines<StdinLock>, exrays: &mut HashMap<String, Exray<i64>>, functions: &FuncMap) -> Result<(), CommandError> {
    if words.len() > 2 {
        return Err(CommandError::CreateError(String::from("Zero or only one argument expected")));
    }

    let mut is_stdin = true;
    let file;
    let reader;
    let mut reader_it = None;
    if words.len() == 2 {
        is_stdin = false;
        match File::open(&words[1]) {
            Err(e) => return Err(CommandError::FileError(e)),
            Ok(f) => file = f,
        }
        reader = BufReader::new(file);
        reader_it = Some(reader.lines().into_iter());
    }

    let mut r;
    if is_stdin {
        println!("Input the exray numbers on the next line:");
        r = try_line(line_it.next());
    }
    else {
        r = try_line(reader_it.as_mut().unwrap().next());
    }
    let mut nums = Vec::<i64>::new();
    match r {
        Err(e) => return Err(e),
        Ok(parts) => {
            for mut part in parts {
                part = part.trim().to_string();
                if part.len() == 0 {
                    continue;
                }
                let nums_str: Vec<_> = part.trim().split(' ').collect();
                for num in nums_str {
                    if num.len() == 0 {
                        continue;
                    }
                    match num.parse::<i64>() {
                        Err(e) => println!("{} cannot be parsed - {:?}", num, e),
                        Ok(num) => nums.push(num),
                    }
                }
            }
            if nums.len() == 0 {
                return Err(CommandError::CreateError(String::from("No numbers were parsed, no new exray created")));
            }
        },
    }

    if is_stdin {
        println!("Input name for the exray on the next line:");
        r = try_line(line_it.next());
    }
    else {
        r = try_line(reader_it.as_mut().unwrap().next());
    }
    let mut name = String::new();
    match r {
        Err(e) => return Err(e),
        Ok(parts) => {
            if parts.len() == 0 {
                return Err(CommandError::CreateError(String::from("No name in the line")));
            }
            name = parts[0].clone();
            if name.len() == 0 {
                return Err(CommandError::CreateError(String::from("No name in the line")));
            }
        }
    }
    if exrays.contains_key(&name) {
        return Err(CommandError::CreateError(String::from("Exray with that name exists")));
    }

    if is_stdin {
        println!("On next line you can input sum, max and/or min to make the exray calculate these functions:");
        r = try_line(line_it.next());
    }
    else {
        r = try_line(reader_it.as_mut().unwrap().next());
    }
    let mut curr_names = HashSet::<String>::new();
    match r {
        Err(e) => return Err(e),
        Ok(words) => {
            for word in words {
                if functions.contains_key(&word) {
                    curr_names.insert(word);
                }
            }
        }
    }

    let mut curr_functions = Vec::<Func<i64>>::new();
    for fn_name in curr_names {
        curr_functions.push(*functions.get(&fn_name).unwrap());
    }

    exrays.insert(name.to_string(), Exray::<i64>::new(nums, curr_functions));
    println!("Exray successfully added!");
    return Ok(());
}

fn main() {            
    let stdin = io::stdin();
    let mut line_it = stdin.lock().lines();

    let mut exrays = HashMap::<String, Exray<i64>>::new();
    let mut functions: FuncMap = HashMap::<String, Func<i64>>::new();
    fill_functions(&mut functions);

    while let Some(line) = line_it.next() {
        if line.is_err() {
            println!("{:?}",line.err());
            continue;
        }
        let mut iter = line.as_ref().unwrap().split_whitespace();
        let mut words = Vec::<String>::new();
        while let Some(word) = iter.next() {
            words.push(String::from(word));
        }
        if words.len() == 0 {
            continue;
        }
        
        let command_name = &words[0];
        if command_name == "exit" {
            break;
        }
        
        if command_name == "create" {
            match create(words, &mut line_it, &mut exrays, &functions) {
                Err(e) => println!("{:?}", e),
                _ => println!("{}",exrays.len()),
            }
        }
    }
}
/*
fn main() {
    println!("Hello, world!");
    
    let mut e = Exray::<i32>::new(vec![|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => *y + *z,
                }
            },
            Some(x) => {
                match z {
                    None => *x + *y,
                    Some(z) => *x + *y + *z,
                }
            }
        }
    }]);
    e.insert(0, 1);
    e.insert(0, 0);
    e.insert(2, 2);
    e.insert(3, 3);
    
    println!("{:?}", e);
    e[2] = 5;
    println!("{:?}", e);
}*/