extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Complex {
    real: f64,
    imag: f64,
}

fn main() {
    let object = Complex{real: 2.2, imag: 3.2};

    let encoded = json::encode(&object).unwrap();

    println!("Encoded: {}", encoded);
    
    // Don't forget to cast the type!
    let decoded: Complex = json::decode(&encoded).unwrap();

    println!("Decoded: {:?}", decoded);

}
