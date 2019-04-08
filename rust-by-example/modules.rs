mod my_mod {
	fn private_function() {
		println!("Shh I'm private, only called inside `private_function()`");
	}

	pub fn pub_function() {
		println!("Hi I'm a public function `my_mod::function()`");
	}

    pub fn call_priv() {
        println!("I'm about to call private!!");
        private_function();
    }

	pub mod nested {
		pub fn function() {
			println!("I'm nested");
		}
	}
}

fn main() {
    my_mod::pub_function();

    my_mod::nested::function();

    my_mod::call_priv();
}
