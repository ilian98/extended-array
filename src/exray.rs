type Link<T> = Option<Box<ImplicitTreap<T>>>;
   
struct ImplicitTreap<T> {
    cnt: u32, // cnt is the size of the subtree
    y_key: i32,
    value: T,
    value_all: Vec<T>, // value_all stores the functions' values for the subtree

    l: Link<T>,
    r: Link<T>,
}

use std::mem;
fn split<T> (curr: &mut Link<T>, ind: u32, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>) {
    match curr {
        None => {
            *l_part = None;
            *r_part = None;
        },
        Some(ref mut node) => {
            let mut curr_ind = 0u32;
            if node.l.is_some() {
                curr_ind=node.l.as_ref().unwrap().cnt;
            }
            if curr_ind < ind {
                *l_part = mem::replace(&mut *curr, None);
                let mut temp = None; 
                split(&mut l_part.as_mut().unwrap().r, ind-(curr_ind+1), &mut temp, &mut r_part);
                l_part.as_mut().unwrap().r = temp;
            }
            else {
                *r_part = mem::replace(&mut *curr, None);
                let mut temp = None; 
                split(&mut r_part.as_mut().unwrap().l, ind, &mut l_part, &mut temp);
                r_part.as_mut().unwrap().l = temp;
            }
        }
    }

}
fn merge<T> (curr: &mut Link<T>, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>) {
    if l_part.is_none() || r_part.is_none() {
        if l_part.is_some() {
            *curr = mem::replace(&mut *l_part, None);
        }
        else {
            *curr = mem::replace(&mut *r_part, None);
        }
        return ;
    }
    if l_part.as_ref().unwrap().y_key > r_part.as_ref().unwrap().y_key {
        *curr = mem::replace(&mut *l_part, None);
        let mut temp = None;
        merge(&mut temp, &mut curr.as_mut().unwrap().r, &mut r_part);
        curr.as_mut().unwrap().r = temp;
    }
    else {
        *curr = mem::replace(&mut *r_part, None);
        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut curr.as_mut().unwrap().l);
        curr.as_mut().unwrap().l = temp;
    }
}

pub struct Exray<T> { // the name comes from the beginning and ending of extended-array :)
    root: Link<T>,
    functions: Vec<Box<dyn Fn(Option<T>, Option<T>) -> T >>
}

extern crate rand;
use rand::Rng;
impl<T: Copy> Exray<T> {
    pub fn new () -> Self {
        Exray::<T> {
            root: None,
            functions: Vec::<Box<dyn Fn(Option<T>, Option<T>) -> T >>::new(),
        }
    }
    pub fn insert (&mut self, ind: u32, value: &T) {
        let mut l_part = None;
        let mut r_part = None;
        split(&mut self.root, ind, &mut l_part, &mut r_part);

        let mut new_treap = Some(Box::new(ImplicitTreap::<T> {
            cnt: 1,
            y_key: rand::thread_rng().gen::<i32>(),
            value: *value,
            value_all: Vec::<T>::new(),

            l: None,
            r: None,
        }));
        for function in self.functions.iter() {
            new_treap.as_mut().unwrap().value_all.push(function(Some(*value), None));
        }
        let mut temp = None;
        merge(&mut temp, &mut l_part, &mut new_treap);
        merge(&mut self.root, &mut temp, &mut r_part);
    }
}

fn output<'a> (curr: Option<&'a Box<ImplicitTreap<i32>>>) {
    if curr.is_none() {
        return ;
    }
    println!("{}, {}, {}",curr.as_ref().unwrap().cnt,curr.as_ref().unwrap().value,curr.as_ref().unwrap().y_key);
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
