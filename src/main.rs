mod exray;
mod functions;
use exray::*;
use functions::*;
use io::Write;

use std::{fs::File, io::Error};
use std::io::{self, BufRead, BufReader, BufWriter, Lines, StdinLock};

#[derive(Debug)]
pub enum CommandError {
    InputEnd(String),
    IOError(io::Error),
    FileError(io::Error),
    ExrayError(ExrayError),

    CreateError(String),
    SaveError(String),
    PrintError(String),
    
    GetElementError(String),
    ChangeElementError(String),
    InsertElementError(String),
    EraseElementError(String),
    EraseSegmentError(String),
    ExtractCloneSegmentError(String),
    InsertExrayError(String),
    SegmentFvaluesError(String),
    ExrayFvalues(String),
}

fn try_line (result: Option<Result<String, Error>>) -> Result<Vec<String>, CommandError> {
    match result {
        None => return Err(CommandError::InputEnd(String::from("Input ended without reading the numbers"))),
        Some(line) => {
            if line.is_err() {
                return Err(CommandError::IOError(line.unwrap_err()));
            }
            let parts: Vec<String> = line.as_ref().unwrap().trim().split(',').map(|s| s.to_string()).collect();
            let mut result = Vec::<String>::new();
            for part in parts {
                let mut v: Vec<String> = part.trim().split(' ').map(|s| s.to_string()).collect();
                result.append(&mut v);
            }
            return Ok(result);
        }
    }
}

use std::collections::{HashSet, HashMap};
type FuncMap = HashMap::<String, Func<i64>>;
type ExrayMap = HashMap::<String, Exray<i64>>;

fn create (words: &[String], line_it: &mut Lines<StdinLock>, exrays: &mut ExrayMap, functions: &FuncMap) -> Result<String, CommandError> {
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
            for num_str in parts {
                if num_str.len() == 0 {
                    continue;
                }
                match num_str.parse::<i64>() {
                    Err(e) => println!("{} cannot be parsed - {:?}", num_str, e),
                    Ok(num) => nums.push(num),
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
    let name;
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
    let mut curr_names = Vec::<String>::new();
    let mut names = HashSet::<String>::new();
    match r {
        Err(e) => return Err(e),
        Ok(words) => {
            for word in words {
                if names.contains(&word) {
                    continue;
                }
                if functions.contains_key(&word) {
                    curr_names.push(word.clone());
                    names.insert(word);
                }
            }
        }
    }

    let mut curr_functions = Vec::<Func<i64>>::new();
    for fn_name in curr_names {
        curr_functions.push(*functions.get(&fn_name).unwrap());
    }

    exrays.insert(name.to_string(), Exray::<i64>::new(nums, curr_functions));
    return Ok(name);
}

fn check_name (words: &[String], expected_len: usize, exrays: &ExrayMap) -> Result<(), Option<String>> {
    if words.len() != expected_len {
        return Err(None);
    }
    let name = &words[1];
    if exrays.contains_key(name) == false {
        return Err(Some(String::from("No exray with that name")));    
    }
    return Ok(());
}

fn save (words: &[String], exrays: &ExrayMap, functions: &FuncMap) -> Result<(), CommandError> {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::SaveError(String::from("Two arguments expected - name of exray and name of file for saving")));
        },
        Err(Some(e)) => return Err(CommandError::SaveError(e)),
        _ => {},
    }
    
    let file;
    match File::create(&words[2]) {
        Err(e) => return Err(CommandError::FileError(e)),
        Ok(f) => file = f,
    }
    let mut writer = BufWriter::new(&file);

    let exray = exrays.get(&words[1]).unwrap();
    for num in exray.to_vec() {
        match write!(writer, "{} ", num) {
            Err(e) => return Err(CommandError::IOError(e)),
            _ => {},
        }
    }
    match write!(writer, "\n{}\n", words[1]) {
        Err(e) => return Err(CommandError::IOError(e)),
        _ => {},
    }
    for exray_func in exray.functions() {
        for (fn_name, func) in functions {
            if *exray_func as usize == *func as usize {
                match write!(writer, "{} ", fn_name) {
                    Err(e) => return Err(CommandError::IOError(e)),
                    _ => {},
                }
                break;
            }
        }
    }
    match writeln!(writer) {
        Err(e) => return Err(CommandError::IOError(e)),
        _ => {},
    }

    return Ok(());
}

fn print_exray (words: &[String], exrays: &ExrayMap, functions: &FuncMap) -> Result<(), CommandError> {
    match check_name(words,2,exrays) {
        Err(None) => {
            return Err(CommandError::PrintError(String::from("One arguments expected - name of exray")));
        },
        Err(Some(e)) => return Err(CommandError::PrintError(e)),
        _ => {},
    }
    
    let exray = exrays.get(&words[1]).unwrap();
    for num in exray.to_vec() {
        print!("{} ", num);
    }
    print!("\n{}\n", words[1]);
    for exray_func in exray.functions() {
        for (fn_name, func) in functions {
            if *exray_func as usize == *func as usize {
                print!("{} ", fn_name);
                break;
            }
        }
    }
    println!();

    return Ok(());
}


fn get_element (words: &[String], exrays: &ExrayMap) -> Result<i64, CommandError> {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::GetElementError(String::from("Two arguments expected - name of exray and index of element")));
        },
        Err(Some(e)) => return Err(CommandError::GetElementError(e)),
        _ => {},
    }
    let index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::GetElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => index = num,
    }
    let exray = exrays.get(&words[1]).unwrap();
    if index >= exray.len() {
        return Err(CommandError::GetElementError(String::from("Index is greater than last index")));
    }

    return Ok(exray[index]);
}

fn change_element (words: &[String], exrays: &mut ExrayMap) -> Result<(), CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::ChangeElementError(String::from("Three arguments expected - name of exray, index of element and new value")));
        },
        Err(Some(e)) => return Err(CommandError::ChangeElementError(e)),
        _ => {},
    }
    let index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ChangeElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => index = num,
    }
    let exray = exrays.get_mut(&words[1]).unwrap();
    if index >= exray.len() {
        return Err(CommandError::ChangeElementError(String::from("Index is greater than last index")));
    }

    let new_value;
    match words[3].parse::<i64>() {
        Err(_) => {
            return Err(CommandError::ChangeElementError(String::from("New value cannot be parsed as i64")))
        },
        Ok(num) => new_value = num,
    }
    
    exray[index] = new_value;
    return Ok(());
}

fn insert_element (words: &[String], exrays: &mut ExrayMap) -> Result<(), CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::InsertElementError(String::from("Three arguments expected - name of exray, index of inserted element and value")));
        },
        Err(Some(e)) => return Err(CommandError::InsertElementError(e)),
        _ => {},
    }
    let index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::InsertElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => index = num,
    }

    let value;
    match words[3].parse::<i64>() {
        Err(_) => {
            return Err(CommandError::InsertElementError(String::from("New value cannot be parsed as i64")))
        },
        Ok(num) => value = num,
    }
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.insert(index,value) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn erase_element (words: &[String], exrays: &mut ExrayMap) -> Result<(), CommandError> {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::EraseElementError(String::from("Two arguments expected - name of exray and index of erased element")));
        },
        Err(Some(e)) => return Err(CommandError::EraseElementError(e)),
        _ => {},
    }
    let index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => index = num,
    }
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.erase(index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn erase_segment (words: &[String], exrays: &mut ExrayMap) -> Result<(), CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::EraseSegmentError(String::from("Three arguments expected - name of exray, begin index of segment and end index of segment")));
        },
        Err(Some(e)) => return Err(CommandError::EraseSegmentError(e)),
        _ => {},
    }
    let beg_index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseSegmentError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => beg_index = num,
    }
    let end_index;
    match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseSegmentError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => end_index = num,
    }
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.erase_segment(beg_index, end_index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn extract_or_clone_segment (words: &[String], exrays: &mut ExrayMap, extract_or_clone: String) -> Result<String, CommandError> {
    match check_name(words,5,exrays) {
        Err(None) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("Four arguments expected - name of exray, begin index of segment, end index of segment and name of new exray")));
        },
        Err(Some(e)) => return Err(CommandError::ExtractCloneSegmentError(e)),
        _ => {},
    }
    let beg_index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => beg_index = num,
    }
    let end_index;
    match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => end_index = num,
    }
    let new_name = words[4].clone();
    if exrays.contains_key(&new_name) {
        return Err(CommandError::ExtractCloneSegmentError(String::from("Exray with the destination name already exists")))
    }
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    let r;
    if extract_or_clone == "extract" {
        r = exray.extract_segment(beg_index, end_index);
    }
    else {
        r = exray.clone_segment(beg_index, end_index);
    } 
    match r {
        Err(e) => return Err(CommandError::ExrayError(e)),
        Ok(new_exray) => {
            exrays.insert(new_name.clone(),new_exray);
            return Ok(new_name);
        },
    }
}

fn insert_exray (words: &[String], exrays: &mut ExrayMap) -> Result<String, CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::InsertExrayError(String::from("Three arguments expected - name of exray to be inserted, name of the destination exray and index")));
        },
        Err(Some(e)) => return Err(CommandError::InsertExrayError(e)),
        _ => {},
    }
    let name_dest = words[2].clone();
    if exrays.contains_key(&name_dest) == false {
        return Err(CommandError::InsertExrayError(String::from("No exray with the destination name")))
    }

    let index;
    match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::InsertExrayError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => index = num,
    }

    let mut exray_source = exrays.remove(&words[1]).unwrap();
    let exray_dest = exrays.get_mut(&name_dest).unwrap();
    match exray_dest.insert_exray(&mut exray_source, index) {
        Err(e) => {
            exrays.insert(words[1].clone(), exray_source);
            return Err(CommandError::ExrayError(e));
        },
        _ => {
            exrays.remove(&words[1]);
            return Ok(words[1].clone());
        },
    }
}

fn segment_fvalues (words: &[String], exrays: &mut ExrayMap, functions: &FuncMap) -> Result<Vec<(String, i64)>, CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::SegmentFvaluesError(String::from("Three arguments expected - name of exray, begin index of segment and end index of segment")));
        },
        Err(Some(e)) => return Err(CommandError::SegmentFvaluesError(e)),
        _ => {},
    }
    let beg_index;
    match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::SegmentFvaluesError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => beg_index = num,
    }
    let end_index;
    match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::SegmentFvaluesError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => end_index = num,
    }
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.segment_functions_values(beg_index, end_index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        Ok(values) => {
            let mut fvalues = Vec::<(String, i64)>::new();
            let len = values.len();
            let exray_funcs = exray.functions();
            for i in 0..len {
                for (fn_name, func) in functions {
                    if exray_funcs[i] as usize == *func as usize {
                        fvalues.push((fn_name.clone(),values[i]));
                        break;
                    }
                }
            }
            return Ok(fvalues);
        },
    }
}

fn exray_fvalues (words: &[String], exrays: &ExrayMap, functions: &FuncMap) -> Result<Vec<(String, i64)>, CommandError> {
    match check_name(words,2,exrays) {
        Err(None) => {
            return Err(CommandError::SegmentFvaluesError(String::from("One argument expected - name of exray")));
        },
        Err(Some(e)) => return Err(CommandError::SegmentFvaluesError(e)),
        _ => {},
    }
    
    let exray = exrays.get(&words[1]).unwrap();
    let values = exray.functions_values();
    let mut fvalues = Vec::<(String, i64)>::new();
    let len = values.len();
    let exray_funcs = exray.functions();
    for i in 0..len {
        for (fn_name, func) in functions {
            if exray_funcs[i] as usize == *func as usize {
                fvalues.push((fn_name.clone(),values[i]));
                break;
            }
        }
    }
    return Ok(fvalues);
}

fn main() {            
    let stdin = io::stdin();
    let mut line_it = stdin.lock().lines();

    let mut exrays = HashMap::<String, Exray<i64>>::new();
    let mut functions = HashMap::<String, Func<i64>>::new();
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
        
        let command_name = &words[0].to_lowercase();
        if command_name == "exit" {
            break;
        }
        
        if command_name == "create" {
            match create(&words, &mut line_it, &mut exrays, &functions) {
                Err(e) => println!("{:?}", e),
                Ok(name) => println!("Exray with name - {}, successfully added!", name),
            }
        }
        else if command_name == "save" {
            match save(&words, &exrays, &functions) {
                Err(e) => println!("{:?}", e),
                _ => println!("Exray successfully saved in file!"),
            }
        }
        else if command_name == "exray_names" {
            if exrays.len() == 0 {
                println!("No exrays");
                continue;
            }
            print!("Exray names:");
            for (name, _) in &exrays {
                print!(" {}", name);
            }
            println!();
        }
        else if command_name == "print" {
            match print_exray(&words, &exrays, &functions) {
                Err(e) => println!("{:?}", e),
                _ => {},
            }
        }

        else if command_name == "get_element" {
            match get_element(&words, &exrays) {
                Err(e) => println!("{:?}", e),
                Ok(element) => println!("Element is - {}", element),
            }
        }
        else if command_name == "change_element" {
            match change_element(&words, &mut exrays) {
                Err(e) => println!("{:?}", e),
                Ok(_) => println!("Element changed successfully!"),
            }
        }
        else if command_name == "insert_element" {
            match insert_element(&words, &mut exrays) {
                Err(e) => println!("{:?}", e),
                Ok(_) => println!("Element inserted successfully!"),
            }
        }
        else if command_name == "erase_element" {
            match erase_element(&words, &mut exrays) {
                Err(e) => println!("{:?}", e),
                Ok(_) => println!("Element erased successfully!"),
            }
        }
        else if command_name == "erase_segment" {
            match erase_segment(&words, &mut exrays) {
                Err(e) => println!("{:?}", e),
                Ok(_) => println!("Segment erased successfully!"),
            }
        }
        else if command_name == "extract_segment" {
            match extract_or_clone_segment(&words, &mut exrays, String::from("extract")) {
                Err(e) => println!("{:?}", e),
                Ok(name) => println!("Segment extracted successfully as exray with name - {}", name),
            }
        }
        else if command_name == "insert_exray" {
            match insert_exray(&words, &mut exrays) {
                Err(e) => println!("{:?}", e),
                Ok(name) => println!("Exray with name {}, inserted successfully and removed from exrays!", name),
            }
        }
        else if command_name == "clone_segment" {
            match extract_or_clone_segment(&words, &mut exrays,String::from("clone")) {
                Err(e) => println!("{:?}", e),
                Ok(name) => println!("Segment cloned successfully as exray with name - {}", name),
            }
        }
        else if command_name == "segment_fvalues" {
            match segment_fvalues(&words, &mut exrays, &functions) {
                Err(e) => println!("{:?}", e),
                Ok(fvalues) => println!("Segment function values - {:?}", fvalues),
            }
        }
        else if command_name == "exray_fvalues" {
            match exray_fvalues(&words, &exrays, &functions) {
                Err(e) => println!("{:?}", e),
                Ok(fvalues) => println!("Exray function values for all numbers - {:?}", fvalues),
            }
        }
        else {
            println!("No command with that name, command names are - exit, create, save, exray_names, print, get_element, change_element, insert_element, erase_element, erase_segment, extract_segment, insert_exray, clone_segment, segment_fvalues, exray_fvalues");
        }
    }
}