// Generated macro for impl_876 (impl)
macro_rules! Depcrate_unordimpl_876 {
() => {
// Module: crate::unord
// Provides: {"impl_876"}
// Dependencies: {}
impl < K , Q : ? Sized , V > Index < & Q > for UnordMap < K , V > where K : Eq + Hash + Borrow < Q > , Q : Eq + Hash , { type Output = V ; # [inline] fn index (& self , key : & Q) -> & V { & self . inner [key] } }
};
}
