mod exray;
use exray::Exray;

fn main() {
    println!("Hello, world!");
    
    let mut e = Exray::<i32>::new(vec![Box::new(|x: Option<&i32>, y: Option<&i32>| -> i32 {
        match x {
            None => {
                match y {
                    None => panic!("Function called with two none as parameters!"),
                    Some(y) => *y,
                }
            },
            Some(x) => {
                match y {
                    None => *x,
                    Some(y) => *x+*y,
                }
            }
        }
    })]);
    e.insert(0, &1);
    e.insert(0, &0);
    e.insert(2, &2);
    e.insert(3, &3);
    e.output();
}
