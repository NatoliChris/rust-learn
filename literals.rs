fn main() {

    /* Important note:
     *      1u32 and 1i32 note difference:
     */
    println!("1 + 2 = {}", 1u32 + 2);
    println!("[i]: 1 - 2 = {}", 1i32 - 2);
    
    // Bitwise
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0011 is {:04b}", 0b0011 | 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //Readability:
    println!("1 million easy to read = {}", 1_000_000u32);

}
