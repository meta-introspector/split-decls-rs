// Generated macro for append (function)
macro_rules! Depcrate_builderappend {
() => {
// Module: crate::builder
// Provides: {"append"}
// Dependencies: {}
fn append (mut dst : & mut dyn Write , header : & Header , mut data : & mut dyn Read) -> io :: Result < () > { dst . write_all (header . as_bytes ()) ? ; let len = io :: copy (& mut data , & mut dst) ? ; pad_zeroes (& mut dst , len) ? ; Ok (()) }
};
}
