use crate::exray::treap::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    use std::cmp;
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

    #[test]
    fn test_insert() {
        let mut e = Exray::<i32>::new(vec![Box::new(|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
            match x {
                None => {
                    match z {
                        None => (*y)*2,
                        Some(z) => (*y)*2 + *z,
                    }
                },
                Some(x) => {
                    match z {
                        None => *x + (*y)*2,
                        Some(z) => *x + (*y)*2 + *z,
                    }
                }
            }
        }), Box::new(|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
            match x {
                None => {
                    match z {
                        None => (*y),
                        Some(z) => cmp::min(*y, *z),
                    }
                },
                Some(x) => {
                    match z {
                        None => cmp::min(*x, *y),
                        Some(z) => cmp::min(cmp::min(*x, *y), *z),
                    }
                }
            }
        })]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_eq!(e.len(), 4);
        //assert_eq!(e.root.as_ref().unwrap().value_all, vec![20,1]);

        assert_match!(e.insert(5, 1).err(), Some(ExrayError::IndexError(_)));
    }

    #[test]
    fn test_erase() {
        let mut e = Exray::<i32>::new(vec![Box::new(|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
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
        })]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_match!(e.erase(0).err(), None); // array should be: [2, 3, 4]
        assert_eq!(e.len(), 3);
        //assert_eq!(e.root.as_ref().unwrap().value_all, vec![9]);
        
        assert_match!(e.erase(1).err(), None); // array should be: [2, 4]
        assert_eq!(e.len(), 2);
        //assert_eq!(e.root.as_ref().unwrap().value_all, vec![6]);

        assert_match!(e.erase(1).err(), None); // array should be: [2]
        assert_eq!(e.len(), 1);
        //assert_eq!(e.root.as_ref().unwrap().value_all, vec![2]);

        assert_match!(e.erase(0).err(), None); // array should be: []
        assert_eq!(e.root.is_none(), true);
        assert_eq!(e.len(), 0);
        

        assert_eq!(e.root.is_none(), true);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_eq!(e.len(), 4);
        //assert_eq!(e.root.as_ref().unwrap().value_all, vec![10]);
        assert_match!(e.erase(4).err(), Some(ExrayError::IndexError(_)));
    }
}

pub struct Exray<T> { // the name comes from the beginning and ending of extended-array :)
    root: Link<T>,
    functions: Vec<Box<Func<T>>>
}
#[derive(Debug)]
pub enum ExrayError {
    IndexError(String),
}

impl<T> Exray<T> {
    pub fn new (functions: Vec<Box<Func<T>>>) -> Self {
        Exray::<T> {
            root: None,
            functions: functions,
        }
    }

    pub fn len (&self) -> usize {
        get_cnt(&self.root) as usize
    }

    pub fn insert (&mut self, ind: usize, value: T) -> Result <(), ExrayError> {
        if self.len() < ind {
            return Err(ExrayError::IndexError(String::from("Index greater than size!")))
        }
        
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind as u32, &mut l_part, &mut r_part, &self.functions);
        /*if l_part.is_some() {
            print!("l: {} ",l_part.as_ref().unwrap().cnt);
        }
        if r_part.is_some() {
            print!("r: {}",r_part.as_ref().unwrap().cnt);
        }
        println!("");*/

        let mut value_all = Vec::<T>::new();
        for function in self.functions.iter() {
            value_all.push(function(None, &value, None));
        }
        let mut new_treap = make_treap(value, value_all);
        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut new_treap, &self.functions);
        merge(&mut self.root, &mut temp, &mut r_part, &self.functions);

        return Ok(());
    }

    pub fn erase (&mut self, ind: usize) -> Result<(), ExrayError> {
        if self.len() <= ind {
            return Err(ExrayError::IndexError(String::from("Index greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind as u32, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, 1, &mut rl_part, &mut rr_part, &self.functions);

        merge(&mut self.root, &mut l_part, &mut rr_part, &self.functions);

        return Ok(());
    }
}

use std::ops::Index;
impl<T> Index<usize> for Exray<T> {
    type Output = T;

    fn index(&self, ind: usize) -> &Self::Output {
        find_index(&self.root, ind as u32)
    }
}
use std::ops::IndexMut;
impl<T> IndexMut<usize> for Exray<T> {
    fn index_mut(&mut self, ind: usize) -> &mut Self::Output {
        find_mut_index(&mut self.root, ind as u32)
    }
}


impl Exray<i32> {
    pub fn output (&mut self) {
        output(self.root.as_ref());
    }
}
