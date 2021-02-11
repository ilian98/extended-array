mod exray;
use exray::Exray;

fn main() {
    println!("Hello, world!");
    
    let mut e = Exray::<i32>::new();
    e.insert(0, &0);
    e.insert(1, &1);
    e.insert(2, &2);
    e.insert(3, &3);
    e.output();
}
