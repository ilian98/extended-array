mod exray;
use exray::Exray;

fn main() {
    println!("Hello, world!");
    
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
    e.insert(0, &1);
    e.insert(0, &0);
    e.insert(2, &2);
    e.insert(3, &3);
    e.output();
}
