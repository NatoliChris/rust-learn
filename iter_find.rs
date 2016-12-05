fn main() {

    let arr = [1,2,3,4,5];

    println!("Find 2 in array: {:?}", arr.iter().find(|&&x| x == 2));
}
