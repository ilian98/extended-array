use std::cmp;
use std::collections::HashMap;

type Func = fn(Option<&i64>, &i64, Option<&i64>) -> i64;

pub fn fill_functions (functions: &mut HashMap::<String, Func>) {
    functions.insert("sum".to_string(),|x: Option<&i64>, y: &i64, z: Option<&i64>| -> i64 {
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
            },
        }
    });
    
    functions.insert("max".to_string(),|x: Option<&i64>, y: &i64, z: Option<&i64>| -> i64 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => cmp::max(*y, *z),
                }
            },
            Some(x) => {
                match z {
                    None => cmp::max(*x, *y),
                    Some(z) => cmp::max(cmp::max(*x, *y), *z),
                }
            },
        }
    });
    
    functions.insert("min".to_string(),|x: Option<&i64>, y: &i64, z: Option<&i64>| -> i64 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => cmp::min(*y, *z),
                }
            },
            Some(x) => {
                match z {
                    None => cmp::min(*x, *y),
                    Some(z) => cmp::min(cmp::min(*x, *y), *z),
                }
            },
        }
    });
}
