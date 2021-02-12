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
    let mut curr_ind = 0u32;
    if node.l.is_some() {
        curr_ind += node.l.as_ref().unwrap().cnt;
    }
    if node.r.is_some() {
        curr_ind += node.r.as_ref().unwrap().cnt;
    }
    node.cnt = curr_ind + 1;

    let len = functions.len();
    node.value_all.clear();
    for i in 0..len {
        let function = &functions[i];
        let mut value;
        if node.l.is_some() {
            value = function(Some(&node.l.as_ref().unwrap().value_all[i]), Some(&node.value));
        }
        else {
            value = function(None, Some(&node.value));
        }
        if node.r.is_some() {
            value = function(Some(&value), Some(&node.r.as_ref().unwrap().value_all[i]));
        }
        else {
            value = function(Some(&value), None);
        }
        node.value_all.push(value);
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
            let mut curr_ind = 1u32;
            if node.l.is_some() {
                curr_ind = node.l.as_ref().unwrap().cnt;
            }
            if curr_ind <= ind {
                *l_part = mem::replace(&mut *curr, None);
                let mut temp = None; 
                split(&mut l_part.as_mut().unwrap().r, ind-curr_ind, &mut temp, &mut r_part, functions);
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

type Func<T> = dyn Fn(Option<&T>, Option<&T>) -> T;
pub struct Exray<T> { // the name comes from the beginning and ending of extended-array :)
    root: Link<T>,
    functions: Vec<Box<Func<T>>>
}

use rand::Rng;
impl<T: Copy> Exray<T> {
    pub fn new (functions: Vec<Box<Func<T>>>) -> Self {
        Exray::<T> {
            root: None,
            functions: functions,
        }
    }

    pub fn insert (&mut self, ind: u32, value: &T) {
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind, &mut l_part, &mut r_part, &self.functions);
        if l_part.is_some() {
            print!("l: {} ",l_part.as_ref().unwrap().cnt);
        }
        if r_part.is_some() {
            print!("r: {}",r_part.as_ref().unwrap().cnt);
        }
        println!("");
        let mut new_treap = Some(Box::new(ImplicitTreap::<T> {
            cnt: 1,
            y_key: rand::thread_rng().gen::<i32>(),
            value: *value,
            value_all: Vec::<T>::new(),

            l: None,
            r: None,
        }));
        for function in self.functions.iter() {
            new_treap.as_mut().unwrap().value_all.push(function(Some(value), None));
        }
        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut new_treap, &self.functions);
        merge(&mut self.root, &mut temp, &mut r_part, &self.functions);
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
