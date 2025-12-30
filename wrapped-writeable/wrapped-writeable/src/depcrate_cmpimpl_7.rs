// Generated macro for impl_7 (impl)
macro_rules! Depcrate_cmpimpl_7 {
() => {
// Module: crate::cmp
// Provides: {"impl_7"}
// Dependencies: {}
# [doc = " This is an infallible impl. Functions always return Ok, not Err."] impl fmt :: Write for WriteComparator < '_ > { # [inline] fn write_str (& mut self , other : & str) -> fmt :: Result { if self . result != Ordering :: Equal { return Ok (()) ; } let (this , remainder) = self . code_units . split_at_checked (other . len ()) . unwrap_or ((self . code_units , & [])) ; self . code_units = remainder ; self . result = this . cmp (other . as_bytes ()) ; Ok (()) } }
};
}
