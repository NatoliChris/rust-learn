fn function() {
    println!("Called outer function()");
}

mod cool {
    pub fn function() {
        println!("Called cool::function()");
    }
}

mod my {
    fn function() {
        println!("Called my::function()");

    }

    mod cool {
        pub fn function() {
            println!("Called my::cool::function()");
        }
    }

    pub fn indirect() {
        println!("Called my::indirect()");
        
        self::function();
        function();

        self::cool::function();

        super::function();
        super::cool::function();

        cool::function();

    }
}


fn main() {
    my::indirect();
}
