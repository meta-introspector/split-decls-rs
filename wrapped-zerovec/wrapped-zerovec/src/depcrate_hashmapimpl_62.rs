// Generated macro for impl_62 (impl)
macro_rules! Depcrate_hashmapimpl_62 {
() => {
// Module: crate::hashmap
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , K , V > ZeroHashMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , { # [doc = " The number of elements in the [`ZeroHashMap`]."] pub fn len (& self) -> usize { self . values . zvl_len () } # [doc = " Whether the [`ZeroHashMap`] is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
