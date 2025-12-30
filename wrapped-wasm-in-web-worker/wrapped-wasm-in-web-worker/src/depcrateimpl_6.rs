// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
# [wasm_bindgen] impl NumberEval { # [doc = " Create new instance."] pub fn new () -> NumberEval { NumberEval { number : 0 } } # [doc = " Check if a number is even and store it as last processed number."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `number` - The number to be checked for being even/odd."] pub fn is_even (& mut self , number : i32) -> bool { self . number = number ; self . number % 2 == 0 } # [doc = " Get last number that was checked - this method is added to work with"] # [doc = " statefulness."] pub fn get_last_number (& self) -> i32 { self . number } }
};
}
