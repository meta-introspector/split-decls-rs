// Generated macro for fill_background_to_right (function)
macro_rules! Depcratefill_background_to_right {
() => {
// Module: crate
// Provides: {"fill_background_to_right"}
// Dependencies: {}
pub fn fill_background_to_right (mut s : String , entire_width : u16) -> String { match (s . len () , entire_width as usize) { (x , y) if x >= y => s , (x , y) => { s . extend (repeat (' ') . take (y - x)) ; s } } }
};
}
