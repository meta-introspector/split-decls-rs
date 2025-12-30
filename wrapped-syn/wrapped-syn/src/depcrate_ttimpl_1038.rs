// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_ttimpl_1038 {
() => {
// Module: crate::tt
// Provides: {"impl_1038"}
// Dependencies: {}
impl < 'a > Hash for TokenStreamHelper < 'a > { fn hash < H : Hasher > (& self , state : & mut H) { let tokens = self . 0 . clone () . into_iter () ; tokens . clone () . count () . hash (state) ; for tt in tokens { TokenTreeHelper (& tt) . hash (state) ; } } }
};
}
