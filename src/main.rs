mod exray;
mod demo;
mod functions;
use exray::*;
use demo::*;
use functions::*;
use io::Write;

use std::{fmt::Display, fs::File, io::Error, str::FromStr};
use std::io::{self, BufRead, BufReader, BufWriter, Lines, StdinLock};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    use std::fs::remove_file;
    macro_rules! assert_match {
        ($expr:expr, $pat:pat) => {
            if let $pat = $expr {
                // all good
            } else {
                assert!(
                    false,
                    "Expression {:?} does not match the pattern {:?}",
                    $expr,
                    stringify!($pat)
                );
            }
        };
    }

    fn write_to_file (file_name: &str, text: &str) {
        match File::create(file_name) {
            Err(_) => panic!("Cannot create file test_create"),
            Ok(file) => {
                let mut writer = BufWriter::new(&file);
                match writeln!(writer, "{}", text) {
                    Err(_) => panic!("Cannot write to file"),
                    _ => {},
                }
            },
        }
    }

    fn check_exray (exray: &Exray<i64, i64>, nums: Vec<i64>, fn_names: Vec<String>, functions: &FuncMap<i64, i64>) -> bool {
        if exray.to_vec() != nums {
            return false;
        }
        let exray_functions = exray.functions();
        let len = fn_names.len();
        if exray_functions.len() != len {
            return false;
        }

        for i in 0..len {
            let expected_func = functions.get(&fn_names[i]).unwrap();
            if exray_functions[i] as usize != *expected_func as usize {
                return false;
            }
        }
        return true;
    }
    fn check_exrays (exrays: &ExrayMap<i64, i64>, expected: Vec<(String, Vec<i64>, Vec<String>)>, functions: &FuncMap<i64, i64>) -> bool {
        if exrays.len() != expected.len() {
            return false;
        }
        for element in expected {
            if exrays.contains_key(&element.0) == false {
                return false;
            }
            let exray = exrays.get(&element.0).unwrap();
            if check_exray(exray, element.1, element.2, functions) == false {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn test_create() {
        let stdin = io::stdin();
        let mut line_it = stdin.lock().lines();
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
        
        write_to_file("test_create", "2 9 -5 10 1024 0\ntest\nsum min max min sum min");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), None);
        assert_eq!(check_exrays(&exrays, vec![(String::from("test"), vec![2, 9, -5, 10, 1024, 0], 
        vec![String::from("sum"), String::from("min"), String::from("max")])], &functions), true);
        
        write_to_file("test_create", "2 9 -5 10 1024 0\ntest\nsum min max min sum min");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::CreateError(_)));
        assert_match!(create(&[], &mut line_it, &mut exrays, &functions).err(), Some(CommandError::CreateError(_)));
        assert_match!(create(&[String::from("1"), String::from("2"), String::from("3")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::CreateError(_)));
        write_to_file("test_create", "2 9 -5 10 1024");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::InputEnd(_)));
        write_to_file("test_create", "2 9 -5 10 1024\ntes");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::InputEnd(_)));
        
        assert_match!(create(&[String::from("create"), String::from("no-file")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::FileError(_)));
        
        write_to_file("test_create", "\ntes\n");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::CreateError(_)));
        write_to_file("test_create", "5\n\n");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), 
        Some(CommandError::CreateError(_)));
        
        write_to_file("test_create", "5\ntest2\nmin");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), None);
        assert_eq!(check_exrays(&exrays, vec![
            (String::from("test"), vec![2, 9, -5, 10, 1024, 0], 
            vec![String::from("sum"), String::from("min"), String::from("max")]),
            (String::from("test2"), vec![5], 
            vec![String::from("min")])], &functions), true);
        

        match remove_file("test_create") {
            Err(_) => panic!("Cannot remove file test_create"),
            _ => {},
        }
    }

    #[test]
    fn test_save() {
        let stdin = io::stdin();
        let mut line_it = stdin.lock().lines();
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
        
        write_to_file("test_create", "2 9 -5 10 1024 0\ntest\nsum min max min sum min");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), None);
        write_to_file("test_create", "5\ntest2\nmin");
        assert_match!(create(&[String::from("create"), String::from("test_create")], &mut line_it, &mut exrays, &functions).err(), None);
        
        assert_match!(save(&[String::from("save"), String::from("test"), String::from("saved")], &mut exrays, &functions).err(), None);
        exrays.remove("test");
        assert_match!(create(&[String::from("create"), String::from("saved")], &mut line_it, &mut exrays, &functions).err(), None);
        assert_eq!(check_exray(exrays.get("test").unwrap(), vec![2, 9, -5, 10, 1024, 0], 
        vec![String::from("sum"), String::from("min"), String::from("max")], &functions), true);

        assert_match!(save(&[String::from("save"), String::from("test2"), String::from("saved")], &mut exrays, &functions).err(), None);
        exrays.remove("test2");
        assert_match!(create(&[String::from("create"), String::from("saved")], &mut line_it, &mut exrays, &functions).err(), None);
        assert_eq!(check_exray(exrays.get("test2").unwrap(), vec![5], 
        vec![String::from("min")], &functions), true);

        assert_match!(save(&[String::from("save"), String::from("no-exray"), String::from("saved")], &mut exrays, &functions).err(), 
        Some(CommandError::SaveError(_)));
        assert_match!(save(&[], &mut exrays, &functions).err(), 
        Some(CommandError::SaveError(_)));
        assert_match!(save(&[String::from("1"), String::from("2"), String::from("3"), String::from("4")], &mut exrays, &functions).err(), 
        Some(CommandError::SaveError(_)));
        
        match remove_file("saved") {
            Err(_) => panic!("Cannot remove file saved"),
            _ => {},
        }
        match remove_file("test_create") {
            Err(_) => panic!("Cannot remove file test_create"),
            _ => {},
        }
    }

    #[test]
    fn test_get_element() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![2,9,-5,10,1024];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        for i in 0..numbers.len() {
            assert_eq!(get_element(&[String::from("1"), String::from("test"), i.to_string()], &exrays).unwrap(), &numbers[i]);
        }

        assert_match!(get_element(&[], &exrays).err(), Some(CommandError::GetElementError(_)));
        assert_match!(get_element(&[String::from("1"), String::from("2"), String::from("3"), String::from("4")], &exrays).err(), 
        Some(CommandError::GetElementError(_)));
        assert_match!(get_element(&[String::from("1"), String::from("no-exray"), 0.to_string()], &exrays).err(), 
        Some(CommandError::GetElementError(_)));
        assert_match!(get_element(&[String::from("1"), String::from("test"), (-1).to_string()], &exrays).err(), 
        Some(CommandError::GetElementError(_)));
        assert_match!(get_element(&[String::from("1"), String::from("test"), 6.to_string()], &exrays).err(), 
        Some(CommandError::GetElementError(_)));
    }

    #[test]
    fn test_change_element() {
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let mut numbers = vec![2,9,-5,10,1024];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![*functions.get("sum").unwrap()]));
        for i in 0..numbers.len() {
            assert_eq!(change_element(&[String::from("1"), String::from("test"), i.to_string(), i.to_string()], &mut exrays).unwrap(), ());
            numbers[i] = i as i64;
            assert_eq!(get_element(&[String::from("1"), String::from("test"), i.to_string()], &exrays).unwrap(), &(i as i64));
            match exrays.get_mut("test").unwrap().recover_fvalues(i) {
                Err(e) => panic!("Error when recovering fvalues - {:?}", e),
                Ok(_) => {},
            }

            let mut sum = 0;
            for num in &numbers {
                sum += *num;
            }
            assert_eq!(exray_fvalues(&[String::from("1"), String::from("test")], &mut exrays, &functions).unwrap(), vec![(String::from("sum"), &sum)]);
        }

        assert_match!(change_element(&[], &mut exrays).err(), Some(CommandError::ChangeElementError(_)));
        assert_match!(change_element(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")], &mut exrays).err(), 
        Some(CommandError::ChangeElementError(_)));
        assert_match!(change_element(&[String::from("1"), String::from("no-exray"), 0.to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::ChangeElementError(_)));
        assert_match!(change_element(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::ChangeElementError(_)));
        assert_match!(change_element(&[String::from("1"), String::from("test"), 0.to_string(), String::from("a")], &mut exrays).err(), 
        Some(CommandError::ChangeElementError(_)));
        assert_match!(change_element(&[String::from("1"), String::from("test"), 6.to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::ChangeElementError(_)));
    }

    #[test]
    fn test_insert_element() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![2,9,-5,10,1024];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        for i in (0..=numbers.len()).rev() {
            assert_eq!(insert_element(&[String::from("1"), String::from("test"), i.to_string(), i.to_string()], &mut exrays).unwrap(), ());
        }
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![0,2,1,9,2,-5,3,10,4,1024,5]);

        assert_match!(insert_element(&[], &mut exrays).err(), Some(CommandError::InsertElementError(_)));
        assert_match!(insert_element(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")], &mut exrays).err(), 
        Some(CommandError::InsertElementError(_)));
        assert_match!(insert_element(&[String::from("1"), String::from("no-exray"), 0.to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::InsertElementError(_)));
        assert_match!(insert_element(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::InsertElementError(_)));
        assert_match!(insert_element(&[String::from("1"), String::from("test"), 0.to_string(), String::from("a")], &mut exrays).err(), 
        Some(CommandError::InsertElementError(_)));
        assert_match!(insert_element(&[String::from("1"), String::from("test"), 12.to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
    }

    #[test]
    fn test_erase_element() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![2,9,-5,10,1024];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        let erase_indices = vec![3, 1, 2, 1, 0];
        let mut len = numbers.len();
        assert_match!(erase_element(&[String::from("1"), String::from("test"), 5.to_string()], &mut exrays).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        for ind in erase_indices {
            assert_eq!(erase_element(&[String::from("1"), String::from("test"), ind.to_string()], &mut exrays).unwrap(), ());
            len = len - 1;
            assert_eq!(exrays.get("test").unwrap().len(), len);
        }
        
        assert_match!(erase_element(&[], &mut exrays).err(), Some(CommandError::EraseElementError(_)));
        assert_match!(erase_element(&[String::from("1"), String::from("2"), String::from("3"), String::from("4")], &mut exrays).err(), 
        Some(CommandError::EraseElementError(_)));
        assert_match!(erase_element(&[String::from("1"), String::from("no-exray"), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::EraseElementError(_)));
        assert_match!(erase_element(&[String::from("1"), String::from("test"), (-1).to_string()], &mut exrays).err(), 
        Some(CommandError::EraseElementError(_)));
    }
    
    #[test]
    fn test_erase_segment() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 5.to_string(), 10.to_string()], &mut exrays).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 5.to_string(), 3.to_string()], &mut exrays).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 3.to_string(), 7.to_string()], &mut exrays).err(), None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![0, 1, 2, 8, 9]);
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 2.to_string(), 4.to_string()], &mut exrays).err(), None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![0, 1]);
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 0.to_string(), 1.to_string()], &mut exrays).err(), None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![]);
        
        assert_match!(erase_segment(&[], &mut exrays).err(), Some(CommandError::EraseSegmentError(_)));
        assert_match!(erase_segment(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")], &mut exrays).err(), 
        Some(CommandError::EraseSegmentError(_)));
        assert_match!(erase_segment(&[String::from("1"), String::from("no-exray"), 0.to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::EraseSegmentError(_)));
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::EraseSegmentError(_)));
        assert_match!(erase_segment(&[String::from("1"), String::from("test"), 0.to_string(), (-1).to_string()], &mut exrays).err(), 
        Some(CommandError::EraseSegmentError(_)));
    }
    
    #[test]
    fn test_extract_or_clone_segment() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        
        assert_match!(extract_or_clone_segment(
        &[String::from("1"), String::from("test"), 3.to_string(), 7.to_string(), String::from("clone")], 
        &mut exrays, String::from("clone")).err(),None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), numbers);
        assert_eq!(exrays.get("clone").unwrap().to_vec(), vec![3, 4, 5, 6, 7]);

        assert_match!(extract_or_clone_segment(
        &[String::from("1"), String::from("test"), 2.to_string(), 5.to_string(), String::from("extract")], 
        &mut exrays, String::from("extract")).err(),None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![0, 1, 6, 7, 8, 9]);
        assert_eq!(exrays.get("clone").unwrap().to_vec(), vec![3, 4, 5, 6, 7]);
        assert_eq!(exrays.get("extract").unwrap().to_vec(), vec![2, 3, 4, 5]);
    
        assert_match!(extract_or_clone_segment(&[], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5"), String::from("6") ], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("no-exray"), 0.to_string(), 0.to_string(), String::from("e")], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), 0.to_string(), 0.to_string(), String::from("clone")], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string(), String::from("e")], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), 0.to_string(), (-1).to_string(), String::from("e")], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), 3.to_string(), 2.to_string(), String::from("e")], &mut exrays, String::from("clone")).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string(), String::from("e")], &mut exrays, String::from("extract")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), 0.to_string(), (-1).to_string(), String::from("e")], &mut exrays, String::from("extract")).err(), 
        Some(CommandError::ExtractCloneSegmentError(_)));
        assert_match!(extract_or_clone_segment(&[String::from("1"), String::from("test"), 3.to_string(), 2.to_string(), String::from("e")], &mut exrays, String::from("extract")).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        
    }

    #[test]
    fn test_insert_exray() {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![0, 1, 2, 3, 8, 9];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        exrays.insert(String::from("test2"), Exray::<i64, i64>::new(vec![4, 5, 6, 7], vec![]));
        
        assert_match!(insert_exray(&[String::from("1"), String::from("test2"), String::from("test"), 4.to_string()], &mut exrays).err(), None);
        assert_eq!(exrays.get("test").unwrap().to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(exrays.contains_key("test2"), false);
        
        assert_match!(insert_exray(&[], &mut exrays).err(), Some(CommandError::InsertExrayError(_)));
        assert_match!(insert_exray(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")], &mut exrays).err(), 
        Some(CommandError::InsertExrayError(_)));
        assert_match!(insert_exray(&[String::from("1"), String::from("no-exray"), String::from("e"), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::InsertExrayError(_)));
        assert_match!(insert_exray(&[String::from("1"), String::from("test"), String::from("no-exray"), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::InsertExrayError(_)));
        assert_match!(insert_exray(&[String::from("1"), String::from("test"), String::from("test"), 0.to_string()], &mut exrays).err(), 
        Some(CommandError::InsertExrayError(_)));
        
        exrays.insert(String::from("test2"), Exray::<i64, i64>::new(vec![4, 5, 6, 7], vec![]));
        assert_match!(insert_exray(&[String::from("1"), String::from("test"), String::from("test2"), (-1).to_string()], &mut exrays).err(), 
        Some(CommandError::InsertExrayError(_)));
        assert_match!(insert_exray(&[String::from("1"), String::from("test"), String::from("test2"), 10.to_string()], &mut exrays).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
    }

    #[test]
    fn test_segment_fvalues() {
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![8, 20, 2, 15, 3, 18, 19, 1, 9, 8];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), 
        vec![*functions.get("sum").unwrap(), *functions.get("max").unwrap(), *functions.get("min").unwrap()]));
        exrays.insert(String::from("test2"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        
        assert_eq!(segment_fvalues(&[String::from("1"), String::from("test"), 2.to_string(), 4.to_string()], &mut exrays, &functions).unwrap(), 
        vec![(String::from("sum"), 20), (String::from("max"), 15), (String::from("min"), 2)]); 
        assert_eq!(segment_fvalues(&[String::from("1"), String::from("test"), 4.to_string(), 8.to_string()], &mut exrays, &functions).unwrap(), 
        vec![(String::from("sum"), 50), (String::from("max"), 19), (String::from("min"), 1)]); 
        assert_eq!(segment_fvalues(&[String::from("1"), String::from("test2"), 4.to_string(), 8.to_string()], &mut exrays, &functions).unwrap(), vec![]); 
        
        assert_match!(segment_fvalues(&[], &mut exrays, &functions).err(), Some(CommandError::SegmentFvaluesError(_)));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")], &mut exrays, &functions).err(), 
        Some(CommandError::SegmentFvaluesError(_)));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("no-exray"), 0.to_string(), 0.to_string()], &mut exrays, &functions).err(), 
        Some(CommandError::SegmentFvaluesError(_)));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("test"), (-1).to_string(), 0.to_string()], &mut exrays, &functions).err(), 
        Some(CommandError::SegmentFvaluesError(_)));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("test"), 0.to_string(), (-1).to_string()], &mut exrays, &functions).err(), 
        Some(CommandError::SegmentFvaluesError(_)));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("test"), 3.to_string(), 10.to_string()], &mut exrays, &functions).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_))));
        assert_match!(segment_fvalues(&[String::from("1"), String::from("test"), 5.to_string(), 3.to_string()], &mut exrays, &functions).err(), 
        Some(CommandError::ExrayError(ExrayError::IndexError(_)))); 
    }

    #[test]
    fn test_exray_fvalues() {
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let numbers = vec![8, 20, 2, 15, 3, 18, 19, 1, 9, 8];
        exrays.insert(String::from("test"), Exray::<i64, i64>::new(numbers.clone(), 
        vec![*functions.get("sum").unwrap(), *functions.get("max").unwrap(), *functions.get("min").unwrap()]));
        exrays.insert(String::from("test2"), Exray::<i64, i64>::new(numbers.clone(), vec![]));
        
        assert_eq!(exray_fvalues(&[String::from("1"), String::from("test")], &exrays, &functions).unwrap(), 
        vec![(String::from("sum"), &103), (String::from("max"), &20), (String::from("min"), &1)]); 
        assert_eq!(exray_fvalues(&[String::from("1"), String::from("test2")], &exrays, &functions).unwrap(), vec![]); 
        
        assert_match!(exray_fvalues(&[], &exrays, &functions).err(), Some(CommandError::ExrayFvaluesError(_)));
        assert_match!(exray_fvalues(&[String::from("1"), String::from("2"), String::from("3")], &exrays, &functions).err(), 
        Some(CommandError::ExrayFvaluesError(_)));
        assert_match!(exray_fvalues(&[String::from("1"), String::from("no-exray")], &exrays, &functions).err(), 
        Some(CommandError::ExrayFvaluesError(_)));
    }
}

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
    ExrayFvaluesError(String),
    ExrayLenError(String),
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
type T = Element; // change to Element for demo!
type FuncMap<T, U> = HashMap::<String, Func<T, U>>;
type ExrayMap<T, U> = HashMap::<String, Exray<T, U>>;

fn create<T, U> (words: &[String], line_it: &mut Lines<StdinLock>, exrays: &mut ExrayMap<T, U>, functions: &FuncMap<T, U>) -> Result<String, CommandError> 
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    if words.len()!= 1 && words.len() != 2 {
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
    let mut nums = Vec::<T>::new();
    match r {
        Err(e) => return Err(e),
        Ok(parts) => {
            for num_str in parts {
                if num_str.len() == 0 {
                    continue;
                }
                match num_str.parse::<T>() {
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

    let mut curr_functions = Vec::<Func<T, U>>::new();
    for fn_name in curr_names {
        curr_functions.push(*functions.get(&fn_name).unwrap());
    }

    exrays.insert(name.to_string(), Exray::<T, U>::new(nums, curr_functions));
    return Ok(name);
}

fn check_name<T, U> (words: &[String], expected_len: usize, exrays: &ExrayMap<T, U>) -> Result<(), Option<String>> {
    if words.len() != expected_len {
        return Err(None);
    }
    let name = &words[1];
    if exrays.contains_key(name) == false {
        return Err(Some(String::from("No exray with that name")));    
    }
    return Ok(());
}

fn save<T, U> (words: &[String], exrays: &ExrayMap<T, U>, functions: &FuncMap<T, U>) -> Result<(), CommandError> 
    where T: Clone + Display {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::SaveError(String::from("Two arguments expected - name of exray and name of file for saving")));
        },
        Err(Some(e)) => return Err(CommandError::SaveError(e)),
        _ => {},
    }
    
    let file = match File::create(&words[2]) {
        Err(e) => return Err(CommandError::FileError(e)),
        Ok(f) => f,
    };
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

fn print_exray<T, U> (words: &[String], exrays: &ExrayMap<T, U>, functions: &FuncMap<T, U>) -> Result<(), CommandError> 
    where T: Clone + Display {
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


fn get_element<'a, T, U> (words: &[String], exrays: &'a ExrayMap<T, U>) -> Result<&'a T, CommandError> {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::GetElementError(String::from("Two arguments expected - name of exray and index of element")));
        },
        Err(Some(e)) => return Err(CommandError::GetElementError(e)),
        _ => {},
    }
    let index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::GetElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    let exray = exrays.get(&words[1]).unwrap();
    if index >= exray.len() {
        return Err(CommandError::GetElementError(String::from("Index is greater than last index")));
    }

    return Ok(&exray[index]);
}

fn change_element<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>) -> Result<(), CommandError> 
    where T: FromStr {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::ChangeElementError(String::from("Three arguments expected - name of exray, index of element and new value")));
        },
        Err(Some(e)) => return Err(CommandError::ChangeElementError(e)),
        _ => {},
    }
    let index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ChangeElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    let exray = exrays.get_mut(&words[1]).unwrap();
    if index >= exray.len() {
        return Err(CommandError::ChangeElementError(String::from("Index is greater than last index")));
    }

    let new_value = match words[3].parse::<T>() {
        Err(_) => {
            return Err(CommandError::ChangeElementError(String::from("New value cannot be parsed")))
        },
        Ok(num) => num,
    };
    
    exray[index] = new_value;
    match exray.recover_fvalues(index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        Ok(_) => return Ok(()),
    }
}

fn insert_element<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>) -> Result<(), CommandError> 
    where T: FromStr {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::InsertElementError(String::from("Three arguments expected - name of exray, index of inserted element and value")));
        },
        Err(Some(e)) => return Err(CommandError::InsertElementError(e)),
        _ => {},
    }
    let index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::InsertElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };

    let value = match words[3].parse::<T>() {
        Err(_) => {
            return Err(CommandError::InsertElementError(String::from("New value cannot be parsed")))
        },
        Ok(num) => num,
    };
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.insert(index, value) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn erase_element<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>) -> Result<(), CommandError> {
    match check_name(words,3,exrays) {
        Err(None) => {
            return Err(CommandError::EraseElementError(String::from("Two arguments expected - name of exray and index of erased element")));
        },
        Err(Some(e)) => return Err(CommandError::EraseElementError(e)),
        _ => {},
    }
    let index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseElementError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.erase(index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn erase_segment<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>) -> Result<(), CommandError> {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::EraseSegmentError(String::from("Three arguments expected - name of exray, begin index of segment and end index of segment")));
        },
        Err(Some(e)) => return Err(CommandError::EraseSegmentError(e)),
        _ => {},
    }
    let beg_index =  match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseSegmentError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    let end_index = match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::EraseSegmentError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.erase_segment(beg_index, end_index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        _ => return Ok(()),
    }
}

fn extract_or_clone_segment<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>, extract_or_clone: String) -> Result<String, CommandError> 
    where T: Clone, U: Clone {
    match check_name(words,5,exrays) {
        Err(None) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("Four arguments expected - name of exray, begin index of segment, end index of segment and name of new exray")));
        },
        Err(Some(e)) => return Err(CommandError::ExtractCloneSegmentError(e)),
        _ => {},
    }
    let beg_index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    let end_index = match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::ExtractCloneSegmentError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
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

fn insert_exray<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>) -> Result<String, CommandError> {
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
    if words[1] == name_dest {
        return Err(CommandError::InsertExrayError(String::from("Exray source and destination name are the same")))
    }

    let index = match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::InsertExrayError(String::from("Index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };

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

fn segment_fvalues<T, U> (words: &[String], exrays: &mut ExrayMap<T, U>, functions: &FuncMap<T, U>) -> Result<Vec<(String, U)>, CommandError> 
    where T: Clone, U: Clone {
    match check_name(words,4,exrays) {
        Err(None) => {
            return Err(CommandError::SegmentFvaluesError(String::from("Three arguments expected - name of exray, begin index of segment and end index of segment")));
        },
        Err(Some(e)) => return Err(CommandError::SegmentFvaluesError(e)),
        _ => {},
    }
    let beg_index = match words[2].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::SegmentFvaluesError(String::from("Begin index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    let end_index = match words[3].parse::<usize>() {
        Err(_) => {
            return Err(CommandError::SegmentFvaluesError(String::from("End index cannot be parsed as usize")))
        },
        Ok(num) => num,
    };
    
    let exray = exrays.get_mut(&words[1]).unwrap();
    match exray.segment_functions_values(beg_index, end_index) {
        Err(e) => return Err(CommandError::ExrayError(e)),
        Ok(values) => {
            let mut fvalues = Vec::<(String, U)>::new();
            let len = values.len();
            let exray_funcs = exray.functions();
            for i in 0..len {
                for (fn_name, func) in functions {
                    if exray_funcs[i] as usize == *func as usize {
                        fvalues.push((fn_name.clone(), values[i].clone()));
                        break;
                    }
                }
            }
            return Ok(fvalues);
        },
    }
}

fn exray_fvalues<'a, T, U> (words: &[String], exrays: &'a ExrayMap<T, U>, functions: &FuncMap<T, U>) -> Result<Vec<(String, &'a U)>, CommandError> {
    match check_name(words,2,exrays) {
        Err(None) => {
            return Err(CommandError::ExrayFvaluesError(String::from("One argument expected - name of exray")));
        },
        Err(Some(e)) => return Err(CommandError::ExrayFvaluesError(e)),
        _ => {},
    }
    
    let exray = exrays.get(&words[1]).unwrap();
    let values = exray.functions_values();
    let mut fvalues = Vec::<(String, &U)>::new();
    let len = values.len();
    let exray_funcs = exray.functions();
    for i in 0..len {
        for (fn_name, func) in functions {
            if exray_funcs[i] as usize == *func as usize {
                fvalues.push((fn_name.clone(), &values[i]));
                break;
            }
        }
    }
    return Ok(fvalues);
}

fn exray_len<T, U> (words: &[String], exrays: &ExrayMap<T, U>) -> Result<(String, usize), CommandError> {
    match check_name(words,2,exrays) {
        Err(None) => {
            return Err(CommandError::ExrayLenError(String::from("One argument expected - name of exray")));
        },
        Err(Some(e)) => return Err(CommandError::ExrayLenError(e)),
        _ => {},
    }
    
    let exray = exrays.get(&words[1]).unwrap();
    return Ok((words[1].clone(), exray.len()));
}

use std::any::type_name;

fn main() {
    let stdin = io::stdin();
    let mut line_it = stdin.lock().lines();

    if type_name::<T>() == "i64" {
        let mut exrays = HashMap::<String, Exray<i64, i64>>::new();
        let mut functions = HashMap::<String, Func<i64, i64>>::new();
        fill_functions_i64(&mut functions);
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
            else if command_name == "exray_len" {
                match exray_len(&words, &exrays) {
                    Err(e) => println!("{:?}", e),
                    Ok((name, len)) => println!("Length of exray {} is {}", name, len),
                }
            }
            else {
                println!("No command with that name, command names are - exit, create, save, exray_names, print, get_element, change_element, insert_element, erase_element, erase_segment, extract_segment, insert_exray, clone_segment, segment_fvalues, exray_fvalues");
            }
        }
    }

    // demo:
    else {
        let data = match get_data() {
            Err(e) => {
                println!("{:?}", e);
                return ;
            }
            Ok(v) => v,
        };
        let mut exrays = HashMap::<String, Exray<Element, (f64, f64)>>::new();
        let mut functions = HashMap::<String, Func<Element, (f64, f64)>>::new();
        fill_functions_element(&mut functions);
        let mut functions_vec = Vec::<Func<Element, (f64, f64)>>::new();
        for (_, func) in &functions {
            functions_vec.push(*func);
        }
        exrays.insert(String::from("corona"), Exray::<Element, (f64, f64)>::new(data, functions_vec));
        println!("Data stored in exray with name corona!");

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
            else if command_name == "exray_len" {
                match exray_len(&words, &exrays) {
                    Err(e) => println!("{:?}", e),
                    Ok((name, len)) => println!("Length of exray {} is {}", name, len),
                }
            }
            else if command_name == "country_segment" {
                match check_name(&words,3, &exrays) {
                    Err(None) => println!("Two argument expected - name of exray, and code of country"),
                    Err(Some(e)) => println!("{:?}", e),
                    _ => {
                        let exray = exrays.get(&words[1]).unwrap();
                        match find_country_segment(words[2].clone(), &exray) {
                            None => println!("No country with that code"),
                            Some((from, to)) => println!("The country segment is from {} to {}", from, to),
                        }
                    },
                }
            }
            else {
                println!("No command with that name, command names are - exit, create, save, exray_names, print, get_element, change_element, insert_element, erase_element, erase_segment, extract_segment, insert_exray, clone_segment, segment_fvalues, exray_fvalues, exray_len, country_segment");
            }
        }
    }
}