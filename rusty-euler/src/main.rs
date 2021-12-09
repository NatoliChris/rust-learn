
// ----------------------------------------
// Problem 1
// ----------------------------------------

/// Problem 1 solution
/// Simple iteration over 1000 then check modulus.
fn prob_1() -> i64 {
    let res = (0..1000)
        .into_iter()
        .filter(|x| *x % 3 == 0 || *x % 5 == 0)
        .reduce(|a, b| { a + b });
    res.unwrap_or(0)
}

// ----------------------------------------
// Problem 2
// ----------------------------------------

/// Fibbonaci Numbers -- Naive implementation
fn fib(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

/// Solution to Problem 2
fn prob_2() -> i64 {
    let mut sum: i64 = 0;
    let mut current_fib: i64 = 0;

    loop {
        let f_num = fib(current_fib);
        if f_num > 4_000_000 {
            break;
        }

        if f_num % 2 == 0 {
            sum += f_num;
        }

        current_fib += 1;
    }

    sum
}


fn main() {
    println!("Problem 1: {}", prob_1());
    println!("Problem 2: {}", prob_2());
}
