fn main() {
    let action = std::env::args().nth(1).expect("Please select an action");
    let item = std::env::args().nth(2).expect("Please select an item");
    
    println!("{:?}, {:?}", action, item);
}
