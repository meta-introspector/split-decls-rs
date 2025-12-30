// Generated macro for impl_168 (impl)
macro_rules! Depcrate_map_databakeimpl_168 {
() => {
// Module: crate::map::databake
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a , K , V > BakeSize for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Container : BakeSize , V :: Container : BakeSize , { fn borrows_size (& self) -> usize { self . keys . borrows_size () + self . values . borrows_size () } }
};
}
