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

    fn exray_to_vec (e: &Exray<i32>) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        let len = e.len();
        for i in 0..len {
            res.push(e[i].clone());
        }
        res
    }
    #[test]
    fn test_insert() {
        let mut e = Exray::<i32>::new(vec![],vec![|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
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
        }, |x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
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
        }]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_eq!(exray_to_vec(&e), vec![1, 2, 3, 4]);
        assert_eq!(e.functions_values(), vec![20,1]);

        assert_match!(e.insert(5, 1).err(), Some(ExrayError::IndexError(_)));
    }

    fn add (x: Option<&i32>, y: &i32, z: Option<&i32>) -> i32 {
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
    }
    #[test]
    fn test_erase() {
        let mut e = Exray::<i32>::new(vec![], vec![add]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_match!(e.erase(0).err(), None);
        assert_eq!(exray_to_vec(&e), vec![2,3,4]);
        assert_eq!(e.functions_values(), vec![9]);
        
        assert_match!(e.erase(1).err(), None);
        assert_eq!(exray_to_vec(&e), vec![2,4]);
        assert_eq!(e.functions_values(), vec![6]);

        assert_match!(e.erase(1).err(), None);
        assert_eq!(exray_to_vec(&e), vec![2]);
        assert_eq!(e.functions_values(), vec![2]);

        assert_match!(e.erase(0).err(), None);
        assert_eq!(exray_to_vec(&e), vec![]);
        

        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 3).err(), None);
        assert_match!(e.insert(3, 4).err(), None);

        assert_eq!(exray_to_vec(&e), vec![1,2,3,4]);
        assert_eq!(e.functions_values(), vec![10]);
        assert_match!(e.erase(4).err(), Some(ExrayError::IndexError(_)));

        assert_match!(e.erase_segment(1,2).err(), None);
        assert_eq!(exray_to_vec(&e), vec![1,4]);
        assert_eq!(e.functions_values(), vec![5]);

        assert_match!(e.erase_segment(1,1).err(), None); // array should be: [1]
        assert_eq!(exray_to_vec(&e), vec![1]);
        assert_eq!(e.functions_values(), vec![1]);

        assert_match!(e.erase_segment(1,1).err(), Some(ExrayError::IndexError(_)));
        assert_match!(e.erase_segment(1,0).err(), Some(ExrayError::IndexError(_)));
        
        assert_match!(e.erase_segment(0,0).err(), None);
        assert_eq!(exray_to_vec(&e), vec![]);
    }

    #[test]
    fn test_other_segment_fns () {
        let mut e = Exray::<i32>::new(vec![],vec![add]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 9).err(), None);
        assert_match!(e.insert(2, 4).err(), None);

        let r = e.extract_segment(1, 2);
        assert_match!(r.as_ref().err(), None);
        let mut new_e = r.unwrap();
        assert_eq!(exray_to_vec(&new_e), vec![2,4]);
        assert_eq!(new_e.functions_values(), vec![6]);
        assert_eq!(new_e.functions.len(), 1);
        assert_eq!(exray_to_vec(&e), vec![1,9]);
        assert_eq!(e.functions_values(), vec![10]);

        assert_match!(new_e.erase(0).err(), None);
        assert_match!(new_e.erase(0).err(), None);
        assert_eq!(exray_to_vec(&e), vec![1,9]);
        assert_eq!(e.functions_values(), vec![10]);

        assert_match!(new_e.insert(0, 2).err(), None);
        assert_match!(new_e.insert(1, 3).err(), None);
        
        let err = e.insert_exray(&mut new_e, 1);
        assert_match!(err.as_ref().err(), None);

        assert_eq!(exray_to_vec(&e), vec![1,2,3,9]);
        assert_eq!(e.functions_values(), vec![15]);
        let r = e.segment_functions_values(1,3);
        assert_match!(r.as_ref().err(), None);
        assert_eq!(r.unwrap(), vec![14]);

        assert_match!(e.segment_functions_values(1,4).err(), Some(ExrayError::IndexError(_)));
        assert_match!(e.segment_functions_values(2,1).err(), Some(ExrayError::IndexError(_)));
        
        let mut fail_e = Exray::<i32>::new(vec![], vec![|x: Option<&i32>, y: &i32, z: Option<&i32>| -> i32 {
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
        let err2 = e.insert_exray(&mut fail_e, 0);
        assert_match!(err2.as_ref().err(), Some(ExrayError::IncompatibleExrayError(_)));
        
        let mut last_e = Exray::<i32>::new(vec![], vec![add]);
        assert_match!(last_e.insert(0, 42).err(), None);
        let err3 = e.insert_exray(&mut last_e, 0); // array should be: [42, 1, 2, 3, 9]
        assert_match!(err3.as_ref().err(), None);
        assert_eq!(e.functions_values(), vec![57]);
        assert_eq!(exray_to_vec(&e), vec![42,1,2,3,9]);
    }

    #[test]
    fn test_clone_segment () {
        let mut e = Exray::<i32>::new(vec![],vec![add]);
        assert_match!(e.insert(0, 2).err(), None);
        assert_match!(e.insert(0, 1).err(), None);
        assert_match!(e.insert(2, 4).err(), None);
        assert_match!(e.insert(3, 9).err(), None);

        let r = e.clone_segment(1, 3);
        assert_match!(r.as_ref().err(), None);
        let mut cloned = r.unwrap();
        assert_eq!(exray_to_vec(&cloned), vec![2,4,9]);
        assert_eq!(cloned.functions_values(), vec![15]);
        assert_eq!(cloned.functions.len(), 1);
        assert_eq!(exray_to_vec(&e), vec![1,2,4,9]);
        assert_eq!(e.functions_values(), vec![16]);

        assert_match!(cloned.erase(2).err(), None);
        assert_eq!(exray_to_vec(&cloned), vec![2,4]);
        assert_eq!(cloned.functions_values(), vec![6]);
        assert_eq!(exray_to_vec(&e), vec![1,2,4,9]);
        assert_eq!(e.functions_values(), vec![16]);

        assert_match!(e.clone_segment(2, 1).err(), Some(ExrayError::IndexError(_)));
        assert_match!(e.clone_segment(2, 4).err(), Some(ExrayError::IndexError(_)));

        let err = e.insert_exray(&mut cloned, 1);
        assert_match!(err.as_ref().err(), None);
        assert_eq!(exray_to_vec(&e), vec![1,2,4,2,4,9]);
    }
}

pub struct Exray<T> { // the name comes from the beginning and ending of extended-array :)
    root: Link<T>,
    functions: Vec<Func<T>>,
}
#[derive(Debug)]
pub enum ExrayError {
    IndexError(String),
    IncompatibleExrayError(String),
}

impl<T> Exray<T> {
    pub fn new (elements: Vec<T>, functions: Vec<Func<T>>) -> Self {
        let mut exray = Exray::<T> {
            root: None,
            functions: functions,
        };
        let mut ind = 0;
        for element in elements {
            match exray.insert(ind, element) {
                Err(_) => return exray,
                _ => ind = ind + 1,
            }
        }
        return exray;
    }

    pub fn len (&self) -> usize {
        get_cnt(&self.root) as usize
    }

    pub fn functions(&self) -> &[Func<T>] {
        &self.functions
    }

    pub fn insert (&mut self, ind: usize, value: T) -> Result <(), ExrayError> {
        if self.len() < ind {
            return Err(ExrayError::IndexError(String::from("Index greater than size!")))
        }
        
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind as u32, &mut l_part, &mut r_part, &self.functions);

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

    pub fn erase_segment (&mut self, beg_ind: usize, end_ind: usize) -> Result<(), ExrayError> {
        if end_ind < beg_ind {
            return Err(ExrayError::IndexError(String::from("End index is smaller than begin index!")))
        }
        if self.len() <= end_ind {
            return Err(ExrayError::IndexError(String::from("End index is greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, beg_ind as u32, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, (end_ind as u32) - (beg_ind as u32) + 1, &mut rl_part, &mut rr_part, &self.functions);

        merge(&mut self.root, &mut l_part, &mut rr_part, &self.functions);

        return Ok(());
    }
    
    pub fn insert_exray (&mut self, source: &mut Self, ind: usize) -> Result <(), ExrayError> {
        if self.len() < ind {
            return Err(ExrayError::IndexError(String::from("Index greater than size!")))
        }
        if self.functions.len() != source.functions.len() {
            return Err(ExrayError::IncompatibleExrayError(String::from("Different number of functions!")))
        }
        let len= self.functions.len();
        for i in 0..len {
            if self.functions[i] as usize != source.functions[i] as usize { // high chance the functions are different
                return Err(ExrayError::IncompatibleExrayError(String::from("Different functions!")))
            }
        }
        
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind as u32, &mut l_part, &mut r_part, &self.functions);

        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut source.root, &self.functions);
        merge(&mut self.root, &mut temp, &mut r_part, &self.functions);

        *source = Self::new(vec![], vec![]);
        return Ok(());
    }


    pub fn extract_segment (&mut self, beg_ind: usize, end_ind: usize) -> Result<Self, ExrayError> {
        if end_ind < beg_ind {
            return Err(ExrayError::IndexError(String::from("End index is smaller than begin index!")))
        }
        if self.len() <= end_ind {
            return Err(ExrayError::IndexError(String::from("End index greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, beg_ind as u32, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, (end_ind as u32) - (beg_ind as u32) + 1, &mut rl_part, &mut rr_part, &self.functions);

        merge(&mut self.root, &mut l_part, &mut rr_part, &self.functions);

        return Ok(Self {
            root: rl_part,
            functions: self.functions.clone(),
        });
    }

    pub fn clone_segment (&mut self, beg_ind: usize, end_ind: usize) -> Result<Self, ExrayError> 
        where T: Clone {
        if end_ind < beg_ind {
            return Err(ExrayError::IndexError(String::from("End index is smaller than begin index!")))
        }
        if self.len() <= end_ind {
            return Err(ExrayError::IndexError(String::from("End index greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, beg_ind as u32, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, (end_ind as u32) - (beg_ind as u32) + 1, &mut rl_part, &mut rr_part, &self.functions);

        let new_root = clone_treap(&rl_part);

        merge(&mut r_part, &mut rl_part, &mut rr_part, &self.functions);
        merge(&mut self.root, &mut l_part, &mut r_part, &self.functions);
        

        return Ok(Self {
            root: new_root,
            functions: self.functions.clone(),
        });
    }
    

    pub fn segment_functions_values (&mut self, beg_ind: usize, end_ind: usize) -> Result<Vec<T>, ExrayError> 
        where T: Clone {
        if end_ind < beg_ind {
            return Err(ExrayError::IndexError(String::from("End index is smaller than begin index!")))
        }
        if self.len() <= end_ind {
            return Err(ExrayError::IndexError(String::from("End index greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, beg_ind as u32, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, (end_ind as u32) - (beg_ind as u32) + 1, &mut rl_part, &mut rr_part, &self.functions);

        let mut values: Vec<T>;
        values = vec![];
        values.extend_from_slice(get_values(&rl_part));

        merge(&mut r_part, &mut rl_part, &mut rr_part, &self.functions);
        merge(&mut self.root, &mut l_part, &mut r_part, &self.functions);
        
        return Ok(values);
    }

    pub fn functions_values (&self) -> &[T] {
        if self.len() == 0 {
            return &[];
        }
        return get_values(&self.root);
    }

    pub fn to_vec (&self) -> Vec<T> 
        where T: Clone {
        let mut result = Vec::<T>::new();
        collect_elements(&self.root, &mut result);
        return result;
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

impl<T> Drop for Exray<T> {
    fn drop(&mut self) {
        drop_treap(&mut self.root);
    }
}

use std::fmt::{self, Debug, Formatter};
impl<T: Clone + Debug> Debug for Exray<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let len = self.len();
        let mut nums = Vec::<T>::new();
        for i in 0..len {
            nums.push(self[i].clone());
        }
        write!(f, "{:?}", nums)
    }
}

impl<T: Clone> Clone for Exray<T> {
    fn clone(&self) -> Self {
        Exray::<T> {
            root: clone_treap(&self.root),
            functions: self.functions.clone(),
        }
    }
}