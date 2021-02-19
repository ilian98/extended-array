/// Here are defined the functions according the function type in treap.rs that are maintained for the default case and the demo case of main
use std::cmp;
use std::collections::HashMap;

use crate::demo::Element;

type Func<T, U> = fn(Option<(&U, u64)>, &T, Option<(&U, u64)>) -> U;

/// this function stores the functions for maintaining sum, max and min in HashMap functions that are used in the default case of main - with integers
pub fn fill_functions_i64(functions: &mut HashMap<String, Func<i64, i64>>) {
    // here we don't need the second value of x and z because it is the number of elements on the left and on the right respectively
    functions.insert(
        "sum".to_string(),
        |x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
            match x {
                None => match z {
                    None => *y,
                    Some(z) => *y + *z.0,
                },
                Some(x) => match z {
                    None => *x.0 + *y,
                    Some(z) => *x.0 + *y + *z.0,
                },
            }
        },
    );

    functions.insert(
        "max".to_string(),
        |x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
            match x {
                None => match z {
                    None => *y,
                    Some(z) => cmp::max(*y, *z.0),
                },
                Some(x) => match z {
                    None => cmp::max(*x.0, *y),
                    Some(z) => cmp::max(cmp::max(*x.0, *y), *z.0),
                },
            }
        },
    );

    functions.insert(
        "min".to_string(),
        |x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
            match x {
                None => match z {
                    None => *y,
                    Some(z) => cmp::min(*y, *z.0),
                },
                Some(x) => match z {
                    None => cmp::min(*x.0, *y),
                    Some(z) => cmp::min(cmp::min(*x.0, *y), *z.0),
                },
            }
        },
    );
}

/// this function stores the functions for maintaining sum, max and min in HashMap functions that are used in the demo case of main - with Element structure containing coronavirus data and returns statistics for cases and deaths
pub fn fill_functions_element(functions: &mut HashMap<String, Func<Element, (f64, f64)>>) {
    // we use f64 because of average
    functions.insert(
        "avg".to_string(),
        |x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
            let yc = y.cases as f64;
            let yd = y.deaths as f64;
            match x {
                None => match z {
                    None => (yc, yd),
                    Some(((zc, zd), cntz)) => { // (zc, zd) should be average values of right part
                        let zcnt = cntz as f64;
                        (
                            (yc + (*zc) * zcnt) / (zcnt + 1.),
                            (yd + (*zd) * zcnt) / (zcnt + 1.),
                        )
                    }
                },
                Some(((xc, xd), cntx)) => {
                    let xcnt = cntx as f64;
                    match z {
                        None => (
                            ((*xc) * xcnt + yc) / (xcnt + 1.),
                            ((*xd) * xcnt + yd) / (xcnt + 1.),
                        ),
                        Some(((zc, zd), cntz)) => {
                            let zcnt = cntz as f64;
                            (
                                ((*xc) * xcnt + yc + (*zc) * zcnt) / (xcnt + 1. + zcnt),
                                ((*xd) * xcnt + yd + (*zd) * zcnt) / (xcnt + 1. + zcnt),
                            )
                        }
                    }
                }
            }
        },
    );

    functions.insert(
        "max".to_string(),
        |x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
            let yc = y.cases;
            let yd = y.deaths;
            match x {
                None => match z {
                    None => (yc as f64, yd as f64),
                    Some(((zc, zd), _)) => (
                        cmp::max(yc, *zc as u64) as f64,
                        cmp::max(yd, *zd as u64) as f64,
                    ),
                },
                Some(((xc, xd), _)) => match z {
                    None => (
                        cmp::max(*xc as u64, yc) as f64,
                        cmp::max(*xd as u64, yd) as f64,
                    ),
                    Some(((zc, zd), _)) => (
                        cmp::max(*xc as u64, cmp::max(yc, *zc as u64)) as f64,
                        cmp::max(*xd as u64, cmp::max(yd, *zd as u64)) as f64,
                    ),
                },
            }
        },
    );

    functions.insert(
        "sum".to_string(),
        |x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
            let yc = y.cases as f64;
            let yd = y.deaths as f64;
            match x {
                None => match z {
                    None => (yc, yd),
                    Some(((zc, zd), _)) => {
                        (
                            yc + (*zc),
                            yd + (*zd),
                        )
                    }
                },
                Some(((xc, xd), _)) => {
                    match z {
                        None => (
                            (*xc) + yc,
                            (*xd) + yd,
                        ),
                        Some(((zc, zd), _)) => {
                            (
                                (*xc) + yc + (*zc),
                                (*xd) + yd + (*zd),
                            )
                        }
                    }
                }
            }
        },
    );
}
