// Generated macro for impl_170 (impl)
macro_rules! Depcrate_map_databakeimpl_170 {
() => {
// Module: crate::map::databake
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , K , V > BakeSize for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , & 'a K :: Slice : BakeSize , & 'a V :: Slice : BakeSize , { fn borrows_size (& self) -> usize { self . keys . borrows_size () + self . values . borrows_size () } }
};
}
