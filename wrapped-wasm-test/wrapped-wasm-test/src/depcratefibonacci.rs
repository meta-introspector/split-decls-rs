// Generated macro for fibonacci (function)
macro_rules! Depcratefibonacci {
() => {
// Module: crate
// Provides: {"fibonacci"}
// Dependencies: {}
fn fibonacci (n : u64) -> u64 { let mut fib : u64 = 1 ; let mut fib1 : u64 = 1 ; let mut fib2 : u64 = 1 ; for _ in 3 ..= n { fib = fib1 + fib2 ; fib1 = fib2 ; fib2 = fib ; } fib }
};
}
