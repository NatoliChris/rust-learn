use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("The slize occupies {} bytes", mem::size_of_val(&slice));
    println!("The slice has {} elements and the first element is {}", slice.len(), slice[0]);
}

fn main() {
    // Define size and type of array
    let xs: [i32; 5] = [1,2,3,4,5];

    //All elements can be initialized to same value
    let xy: [i32; 500] = [10; 500];

    analyze_slice(&xs);
    analyze_slice(&xy);
}
