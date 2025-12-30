// Generated macro for impl_345 (impl)
macro_rules! Depcrate_exprimpl_345 {
() => {
// Module: crate::expr
// Provides: {"impl_345"}
// Dependencies: {}
impl Hash for Member { fn hash < H : Hasher > (& self , state : & mut H) { match self { Member :: Named (m) => m . hash (state) , Member :: Unnamed (m) => m . hash (state) , } } }
};
}
