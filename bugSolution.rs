fn main() {
    let mut v = vec![1, 2, 3];
    let first_element = v.get_mut(0);
    match first_element {
        Some(element) => *element = 10,
        None => println!("Index out of bounds"),
    }
    println!("v: {:?}", v);
}