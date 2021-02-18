use std::cmp;
use std::collections::HashMap;

use crate::demo::Element;

type Func<T, U> = fn(Option<(&U, u64)>, &T, Option<(&U, u64)>) -> U;

pub fn fill_functions_i64 (functions: &mut HashMap::<String, Func<i64, i64>>) {
    functions.insert("sum".to_string(),|x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => *y + *z.0,
                }
            },
            Some(x) => {
                match z {
                    None => *x.0 + *y,
                    Some(z) => *x.0 + *y + *z.0,
                }
            },
        }
    });
    
    functions.insert("max".to_string(),|x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => cmp::max(*y, *z.0),
                }
            },
            Some(x) => {
                match z {
                    None => cmp::max(*x.0, *y),
                    Some(z) => cmp::max(cmp::max(*x.0, *y), *z.0),
                }
            },
        }
    });
    
    functions.insert("min".to_string(),|x: Option<(&i64, u64)>, y: &i64, z: Option<(&i64, u64)>| -> i64 {
        match x {
            None => {
                match z {
                    None => *y,
                    Some(z) => cmp::min(*y, *z.0),
                }
            },
            Some(x) => {
                match z {
                    None => cmp::min(*x.0, *y),
                    Some(z) => cmp::min(cmp::min(*x.0, *y), *z.0),
                }
            },
        }
    });
}

pub fn fill_functions_element (functions: &mut HashMap::<String, Func<Element, (f64, f64)>>) {
    functions.insert("avg".to_string(),|x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
        let yc = y.cases as f64;
        let yd = y.deaths as f64;
        match x {
            None => {
                match z {
                    None => (yc, yd),
                    Some(((zc, zd), cntz)) => {
                        let zcnt = cntz as f64;
                        ((yc + (*zc)*zcnt)/(zcnt + 1.), (yd + (*zd)*zcnt)/(zcnt + 1.))
                    },
                }
            },
            Some(((xc, xd), cntx)) => {
                let xcnt = cntx as f64;
                match z {
                    None => (((*xc)*xcnt + yc)/(xcnt + 1.), ((*xd)*xcnt + yd)/(xcnt + 1.)),
                    Some(((zc, zd), cntz)) => {
                        let zcnt = cntz as f64;
                        (((*xc)*xcnt + yc + (*zc)*zcnt)/(xcnt + 1. + zcnt), 
                        ((*xd)*xcnt + yd + (*zd)*zcnt)/(xcnt + 1. + zcnt))
                    },
                }
            },
        }
    });
    
    functions.insert("max".to_string(),|x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
        let yc = y.cases;
        let yd = y.deaths;
        match x {
            None => {
                match z {
                    None => (yc as f64, yd as f64),
                    Some(((zc, zd), _)) => {
                        (cmp::max(yc,*zc as u64) as f64, cmp::max(yd,*zd as u64) as f64)
                    },
                }
            },
            Some(((xc, xd), _)) => {
                match z {
                    None => (cmp::max(*xc as u64, yc) as f64, cmp::max(*xd as u64, yd) as f64),
                    Some(((zc, zd), _)) => {
                        (cmp::max(*xc as u64,cmp::max(yc,*zc as u64)) as f64, 
                        cmp::max(*xd as u64, cmp::max(yd,*zd as u64)) as f64)
                    },
                }
            },
        }
    });
    
    functions.insert("min".to_string(),|x: Option<(&(f64, f64), u64)>, y: &Element, z: Option<(&(f64, f64), u64)>| -> (f64, f64) {
        let yc = y.cases;
        let yd = y.deaths;
        match x {
            None => {
                match z {
                    None => (yc as f64, yd as f64),
                    Some(((zc, zd), _)) => {
                        (cmp::min(yc,*zc as u64) as f64, cmp::min(yd,*zd as u64) as f64)
                    },
                }
            },
            Some(((xc, xd), _)) => {
                match z {
                    None => (cmp::min(*xc as u64, yc) as f64, cmp::min(*xd as u64, yd) as f64),
                    Some(((zc, zd), _)) => {
                        (cmp::min(*xc as u64,cmp::min(yc,*zc as u64)) as f64, 
                        cmp::min(*xd as u64, cmp::min(yd,*zd as u64)) as f64)
                    },
                }
            },
        }
    });
}