pub type Link<T, U> = Option<Box<ImplicitTreap<T, U>>>;
pub type Func<T, U> = fn(Option<(&U, u64)>, &T, Option<(&U, u64)>) -> U;
pub struct ImplicitTreap<T, U> {
    cnt: u64, // cnt is the size of the subtree
    y_key: i64,
    value: T,
    value_all: Vec<U>, // value_all stores the functions' values for the subtree

    l: Link<T, U>,
    r: Link<T, U>,
}

pub fn get_cnt<T, U> (curr: &Link<T, U>) -> u64 {
    if curr.is_none() {
        return 0;
    }
    return curr.as_ref().unwrap().cnt;
}
pub fn get_values<T, U> (curr: &Link<T, U>) -> &[U] {
    if curr.is_none() {
        return &[];
    }
    return &curr.as_ref().unwrap().value_all;
}

use rand::Rng;
pub fn make_treap<T, U> (value: T, value_all: Vec<U>) -> Link<T, U> {
    Some(Box::new(ImplicitTreap::<T, U> {
        cnt: 1,
        y_key: rand::thread_rng().gen::<i64>(),
        value: value,
        value_all: value_all,

        l: None,
        r: None,
    }))
}

fn recover<T, U> (curr: &mut Link<T, U>, functions: &[Func<T, U>]) {
    if curr.is_none() {
        return ;
    }
    let node = curr.as_mut().unwrap();
    let mut curr_ind = 1;
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
        
        let mut l_data= None;
        if node.l.is_some() {
            l_data = Some((&node.l.as_ref().unwrap().value_all[i], node.l.as_ref().unwrap().cnt));
        }
        let mut r_data = None;
        if node.r.is_some() {
            r_data = Some((&node.r.as_ref().unwrap().value_all[i], node.r.as_ref().unwrap().cnt));
        }
        node.value_all.push(function(l_data, &node.value, r_data));
    }
}
use std::mem;
pub fn split<T, U> (curr: &mut Link<T, U>, ind: u64, mut l_part: &mut Link<T, U>, mut r_part: &mut Link<T, U>,
    functions: &[Func<T, U>]) {
    match curr {
        None => {
            *l_part = None;
            *r_part = None;
        },
        Some(ref node) => {
            let mut curr_len = 1;
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
pub fn merge<T, U> (mut curr: &mut Link<T, U>, mut l_part: &mut Link<T, U>, mut r_part: &mut Link<T, U>, 
    functions: &[Func<T, U>]) {
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

pub fn find_index<T, U> (curr: &Link<T, U>, ind: u64) -> &T {
    let mut curr_len = 1;
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
pub fn find_mut_index<T, U> (curr: &mut Link<T, U>, ind: u64) -> &mut T {
    let mut curr_len = 1;
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

pub fn drop_treap<T, U> (curr: &mut Link<T, U>) {
    if curr.is_none() {
        return ;
    }
    let node = curr.as_mut().unwrap();
    if node.l.is_some() {
        drop_treap(&mut node.l);
    }
    if node.r.is_some() {
        drop_treap(&mut node.r);
    }
    drop(node);
}


pub fn clone_treap<T, U> (curr: &Link<T, U>) -> Link<T, U> 
    where T: Clone, U: Clone {
    if curr.is_none() {
        return None;
    }
    let node = curr.as_ref().unwrap();
    let mut new_node = Box::new(ImplicitTreap::<T, U> {
        cnt: node.cnt,
        y_key: node.y_key,
        value: node.value.clone(),
        value_all: node.value_all.clone(),

        l: None,
        r: None,
    });
    if node.l.is_some() {
        new_node.l = clone_treap(&node.l);
    }
    if node.r.is_some() {
        new_node.r = clone_treap(&node.r);
    }
    drop(node);

    Some(new_node)
}

pub fn collect_elements<T, U> (curr: &Link<T, U>, mut v: &mut Vec<T>) 
    where T: Clone {
    if curr.is_none() {
        return ;
    }
    let node = curr.as_ref().unwrap();
    collect_elements(&node.l, &mut v);
    v.push(node.value.clone());
    collect_elements(&node.r, &mut v);
}