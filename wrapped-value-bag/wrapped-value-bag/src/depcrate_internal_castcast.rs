// Generated macro for Cast (enum)
macro_rules! Depcrate_internal_castCast {
() => {
// Module: crate::internal::cast
// Provides: {"Cast"}
// Dependencies: {}
pub (in crate :: internal) enum Cast < 'v > { Signed (i64) , Unsigned (u64) , BigSigned (i128) , BigUnsigned (u128) , Float (f64) , Bool (bool) , Char (char) , Str (& 'v str) , None , # [cfg (feature = "alloc")] String (String) , }
};
}
