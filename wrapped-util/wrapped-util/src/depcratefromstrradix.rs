// Generated macro for FromStrRadix (trait)
macro_rules! DepcrateFromStrRadix {
() => {
// Module: crate
// Provides: {"FromStrRadix"}
// Dependencies: {}
trait FromStrRadix : Sized { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > ; }
};
}
