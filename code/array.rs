use std::io;

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Enter index : ");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Cannot read input");
    let index:usize = index.trim().parse().expect("please enter a valid number");
    println!("element found at pos: {}",arr[index]);
    
}
