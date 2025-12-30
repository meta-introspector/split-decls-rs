// Generated macro for impl_75 (impl)
macro_rules! Depcrate_to_string_or_borrowimpl_75 {
() => {
// Module: crate::to_string_or_borrow
// Provides: {"impl_75"}
// Dependencies: {}
# [doc = " This is an infallible impl. Functions always return Ok, not Err."] impl fmt :: Write for SliceOrString < '_ > { # [inline] fn write_str (& mut self , other : & str) -> fmt :: Result { match self { SliceOrString :: Slice (slice) => { if ! slice . try_push (other) { let valid_str = slice . validated_as_str () ; let mut owned = String :: with_capacity (valid_str . len () + other . len ()) ; owned . push_str (valid_str) ; owned . push_str (other) ; * self = SliceOrString :: String (owned) ; } Ok (()) } SliceOrString :: String (owned) => owned . write_str (other) , } } }
};
}
