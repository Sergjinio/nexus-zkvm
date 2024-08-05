#![no_std]
#![no_main]

use nexus_rt::read_from_private_input;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[nexus_rt::main]
fn main() {
    let input = read_from_private_input().unwrap_or(3) as u32;
    let _ = fib(input);
    // assert_eq!(result, 2);
}
