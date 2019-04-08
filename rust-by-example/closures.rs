fn call_me<F: Fn()>(f: F) {
    f();
}

fn main() {

    fn incr (i: i32) -> i32 { i+ 1 }

    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("Function increment: {}", incr(i));
    println!("Closure: {}", closure_inferred(i));

    let cl = || println!("I'm a closure!");

    call_me(cl);
}
