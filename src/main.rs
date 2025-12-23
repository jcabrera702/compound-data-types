fn main() {
    let numbers: [i32; 4]= [1,2,3,4];
    println!("Array of numbers {:?}", numbers);
    // let mix = [1, 2, "apple", true];
    // println!("Mixed Array {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);
}
