// Generated macro for tmpname (function)
macro_rules! Depcrate_utiltmpname {
() => {
// Module: crate::util
// Provides: {"tmpname"}
// Dependencies: {}
fn tmpname (rng : & mut fastrand :: Rng , prefix : & OsStr , suffix : & OsStr , rand_len : usize) -> OsString { let capacity = prefix . len () . saturating_add (suffix . len ()) . saturating_add (rand_len) ; let mut buf = OsString :: with_capacity (capacity) ; buf . push (prefix) ; let mut char_buf = [0u8 ; 4] ; for c in repeat_with (| | rng . alphanumeric ()) . take (rand_len) { buf . push (c . encode_utf8 (& mut char_buf)) ; } buf . push (suffix) ; buf }
};
}
