// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < T : Copy > BlackBox < T > { # [doc = " Constructs a new instance of `BlackBox` which will wrap the specified value."] # [doc = ""] # [doc = " All access to the inner value will be mediated by a `black_box` optimization barrier."] pub const fn new (value : T) -> Self { Self (value) } # [doc = " Read the inner value, applying an optimization barrier on access."] pub fn get (self) -> T { black_box (self . 0) } }
};
}
