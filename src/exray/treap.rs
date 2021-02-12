pub type Link<T> = Option<Box<ImplicitTreap<T>>>;
pub type Func<T> = dyn Fn(Option<&T>, &T, Option<&T>) -> T;
pub struct ImplicitTreap<T> {
    cnt: u32, // cnt is the size of the subtree
    y_key: i32,
    value: T,
    value_all: Vec<T>, // value_all stores the functions' values for the subtree

    l: Link<T>,
    r: Link<T>,
}

pub fn get_cnt<T> (curr: &Link<T>) -> u32 {
    if curr.is_none() {
        return 0;
    }
    return curr.as_ref().unwrap().cnt;
}

use rand::Rng;
pub fn make_treap<T> (value: T, value_all: Vec<T>) -> Link<T> {
    Some(Box::new(ImplicitTreap::<T> {
        cnt: 1,
        y_key: rand::thread_rng().gen::<i32>(),
        value: value,
        value_all: value_all,

        l: None,
        r: None,
    }))
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
pub fn split<T> (curr: &mut Link<T>, ind: u32, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>,
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
pub fn merge<T> (mut curr: &mut Link<T>, mut l_part: &mut Link<T>, mut r_part: &mut Link<T>, 
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

pub fn find_index<T> (curr: &Link<T>, ind: u32) -> &T {
    let mut curr_len = 1u32;
    let node = curr.as_ref().unwrap();
    if node.l.is_some() {
        curr_len += node.l.as_ref().unwrap().cnt;
    }
    if curr_len == ind + 1 {
        &node.value
    }
    else if curr_len < ind + 1 {
        find_index(&node.r, ind - curr_len)
    }
    else {
        find_index(&node.l, ind)
    }
}
pub fn find_mut_index<T> (curr: &mut Link<T>, ind: u32) -> &mut T {
    let mut curr_len = 1u32;
    let node = curr.as_mut().unwrap();
    if node.l.is_some() {
        curr_len += node.l.as_ref().unwrap().cnt;
    }
    if curr_len == ind + 1 {
        &mut node.value
    }
    else if curr_len < ind + 1 {
        find_mut_index(&mut node.r, ind - curr_len)
    }
    else {
        find_mut_index(&mut node.l, ind)
    }
}


pub fn output<'a> (curr: Option<&'a Box<ImplicitTreap<i32>>>) {
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
