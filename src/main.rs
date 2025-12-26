fn main() {
    let numbers: [i32; 4]= [1,2,3,4];
    println!("Array of numbers {:?}", numbers);
    // let mix = [1, 2, "apple", true];
    // println!("Mixed Array {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    //SLices
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

     let animal_slices:&[&str; 4] = &["Tiger", "Lion", "Bear", "Rat"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slice:&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Zen".to_string()];
    println!("Book Slice: {:?}", book_slice);
}
