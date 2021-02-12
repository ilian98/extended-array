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
        assert_match!(e.insert(0, &2).err(), None);
        assert_match!(e.insert(0, &1).err(), None);
        assert_match!(e.insert(2, &3).err(), None);
        assert_match!(e.insert(3, &4).err(), None);

        assert_eq!(e.root.as_ref().unwrap().cnt, 4);
        assert_eq!(e.root.as_ref().unwrap().value_all, vec![20,1]);

        assert_match!(e.insert(5, &1).err(), Some(ExrayError::IndexError(_)));
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
        assert_match!(e.insert(0, &2).err(), None);
        assert_match!(e.insert(0, &1).err(), None);
        assert_match!(e.insert(2, &3).err(), None);
        assert_match!(e.insert(3, &4).err(), None);

        assert_match!(e.erase(0).err(), None); // array should be: [2, 3, 4]
        assert_eq!(e.root.as_ref().unwrap().cnt, 3);
        assert_eq!(e.root.as_ref().unwrap().value_all, vec![9]);
        
        assert_match!(e.erase(1).err(), None); // array should be: [2, 4]
        assert_eq!(e.root.as_ref().unwrap().cnt, 2);
        assert_eq!(e.root.as_ref().unwrap().value_all, vec![6]);

        assert_match!(e.erase(1).err(), None); // array should be: [2]
        assert_eq!(e.root.as_ref().unwrap().cnt, 1);
        assert_eq!(e.root.as_ref().unwrap().value_all, vec![2]);

        assert_match!(e.erase(0).err(), None); // array should be: []
        assert_eq!(e.root.is_none(), true);


        assert_match!(e.insert(0, &2).err(), None);
        assert_match!(e.insert(0, &1).err(), None);
        assert_match!(e.insert(2, &3).err(), None);
        assert_match!(e.insert(3, &4).err(), None);

        assert_eq!(e.root.as_ref().unwrap().cnt, 4);
        assert_eq!(e.root.as_ref().unwrap().value_all, vec![10]);
        assert_match!(e.erase(4).err(), Some(ExrayError::IndexError(_)));
    }
}

type Link<T> = Option<Box<ImplicitTreap<T>>>;
   
struct ImplicitTreap<T> {
    cnt: u32, // cnt is the size of the subtree
    y_key: i32,
    value: T,
    value_all: Vec<T>, // value_all stores the functions' values for the subtree

    l: Link<T>,
    r: Link<T>,
}

fn recover<T> (curr: &mut Link<T>, functions: &[Box<Func<T>>]) {
    if curr.is_none() {
        return ;
    }
    let node = curr.as_mut().unwrap();
    let mut curr_ind = 1u32;
    if node.l.is_some() {
        curr_ind += node.l.as_ref().unwrap().cnt;
    }
    if node.r.is_some() {
        curr_ind += node.r.as_ref().unwrap().cnt;
    }
    node.cnt = curr_ind;

    let len = functions.len();
    node.value_all.clear();
    for i in 0..len {
        let function = &functions[i];
        
        let mut l = None; 
        if node.l.is_some() {
            l = Some(&node.l.as_ref().unwrap().value_all[i]);
        }
        let mut r = None;
        if node.r.is_some() {
            r = Some(&node.r.as_ref().unwrap().value_all[i]);
        }
        node.value_all.push(function(l, &node.value, r));
    }
}
use std::mem;
fn split<T> (curr: &mut Link<T>, ind: u32, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>,
    functions: &[Box<Func<T>>]) {
    match curr {
        None => {
            *l_part = None;
            *r_part = None;
        },
        Some(ref node) => {
            let mut curr_len = 1u32;
            if node.l.is_some() {
                curr_len += node.l.as_ref().unwrap().cnt;
            }
            if curr_len <= ind {
                *l_part = mem::replace(&mut *curr, None);
                let mut temp = None; 
                split(&mut l_part.as_mut().unwrap().r, ind - curr_len, &mut temp, &mut r_part, functions);
                l_part.as_mut().unwrap().r = temp;
                recover(&mut l_part, functions);
            }
            else {
                *r_part = mem::replace(&mut *curr, None);
                let mut temp = None; 
                split(&mut r_part.as_mut().unwrap().l, ind, &mut l_part, &mut temp, functions);
                r_part.as_mut().unwrap().l = temp;
                recover(&mut r_part, functions);
            }
        }
    }

}
fn merge<T> (mut curr: &mut Link<T>, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>, 
    functions: &[Box<Func<T>>]) {
    if l_part.is_none() || r_part.is_none() {
        if l_part.is_some() {
            *curr = mem::replace(&mut *l_part, None);
        }
        else {
            *curr = mem::replace(&mut *r_part, None);
        }
        return ;
    }
    let mut temp = None;
    if l_part.as_ref().unwrap().y_key > r_part.as_ref().unwrap().y_key {
        *curr = mem::replace(&mut *l_part, None);
        merge(&mut temp, &mut curr.as_mut().unwrap().r, &mut r_part, functions);
        curr.as_mut().unwrap().r = temp;
    }
    else {
        *curr = mem::replace(&mut *r_part, None);
        merge(&mut temp, &mut l_part, &mut curr.as_mut().unwrap().l, functions);
        curr.as_mut().unwrap().l = temp;
    }
    recover(&mut curr, functions);
}

type Func<T> = dyn Fn(Option<&T>, &T, Option<&T>) -> T;
pub struct Exray<T> { // the name comes from the beginning and ending of extended-array :)
    root: Link<T>,
    functions: Vec<Box<Func<T>>>
}
#[derive(Debug)]
pub enum ExrayError {
    IndexError(String),
}

use rand::Rng;
impl<T: Copy> Exray<T> {
    pub fn new (functions: Vec<Box<Func<T>>>) -> Self {
        Exray::<T> {
            root: None,
            functions: functions,
        }
    }

    pub fn insert (&mut self, ind: u32, value: &T) -> Result <(), ExrayError> {
        if self.root.is_some() && self.root.as_ref().unwrap().cnt < ind {
            return Err(ExrayError::IndexError(String::from("Index greater than size!")))
        }
        
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind, &mut l_part, &mut r_part, &self.functions);
        /*if l_part.is_some() {
            print!("l: {} ",l_part.as_ref().unwrap().cnt);
        }
        if r_part.is_some() {
            print!("r: {}",r_part.as_ref().unwrap().cnt);
        }
        println!("");*/

        let mut new_treap = Some(Box::new(ImplicitTreap::<T> {
            cnt: 1,
            y_key: rand::thread_rng().gen::<i32>(),
            value: *value,
            value_all: Vec::<T>::new(),

            l: None,
            r: None,
        }));
        for function in self.functions.iter() {
            new_treap.as_mut().unwrap().value_all.push(function(None, value, None));
        }
        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut new_treap, &self.functions);
        merge(&mut self.root, &mut temp, &mut r_part, &self.functions);

        return Ok(());
    }

    pub fn erase (&mut self, ind: u32) -> Result<(), ExrayError> {
        if self.root.is_some() && self.root.as_ref().unwrap().cnt <= ind {
            return Err(ExrayError::IndexError(String::from("Index greater than last index!")))
        }

        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind, &mut l_part, &mut r_part, &self.functions);
        let mut rl_part = None;
        let mut rr_part = None;
        split(&mut r_part, 1, &mut rl_part, &mut rr_part, &self.functions);
        assert_eq!(rl_part.as_ref().unwrap().cnt, 1);

        merge(&mut self.root, &mut l_part, &mut rr_part, &self.functions);

        return Ok(());
    }
}

fn output<'a> (curr: Option<&'a Box<ImplicitTreap<i32>>>) {
    if curr.is_none() {
        return ;
    }
    print!("{}, {}, {},  ",curr.as_ref().unwrap().cnt,curr.as_ref().unwrap().value,curr.as_ref().unwrap().y_key);
    for value in curr.as_ref().unwrap().value_all.iter() {
        print!("{} ",value);
    }
    println!("");

    print!("l: ");
    output(curr.as_ref().unwrap().l.as_ref());
    print!("\nr: ");
    output(curr.as_ref().unwrap().r.as_ref());
}
impl Exray<i32> {
    pub fn output (&mut self) {
        output(self.root.as_ref());
    }
}
