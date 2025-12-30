// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
impl LengthHint { pub fn undefined () -> Self { Self (0 , None) } # [doc = " `write_to` will use exactly n bytes."] pub fn exact (n : usize) -> Self { Self (n , Some (n)) } # [doc = " `write_to` will use at least n bytes."] pub fn at_least (n : usize) -> Self { Self (n , None) } # [doc = " `write_to` will use at most n bytes."] pub fn at_most (n : usize) -> Self { Self (0 , Some (n)) } # [doc = " `write_to` will use between `n` and `m` bytes."] pub fn between (n : usize , m : usize) -> Self { Self (Ord :: min (n , m) , Some (Ord :: max (n , m))) } # [doc = " Returns a recommendation for the number of bytes to pre-allocate."] # [doc = " If an upper bound exists, this is used, otherwise the lower bound"] # [doc = " (which might be 0)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use writeable::Writeable;"] # [doc = ""] # [doc = " fn pre_allocate_string(w: &impl Writeable) -> String {"] # [doc = "     String::with_capacity(w.writeable_length_hint().capacity())"] # [doc = " }"] # [doc = " ```"] pub fn capacity (& self) -> usize { self . 1 . unwrap_or (self . 0) } # [doc = " Returns whether the `LengthHint` indicates that the string is exactly 0 bytes long."] pub fn is_zero (& self) -> bool { self . 1 == Some (0) } }
};
}
