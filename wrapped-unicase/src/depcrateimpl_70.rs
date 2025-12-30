// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < S : AsRef < str > > Hash for UniCase < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { match self . 0 { Encoding :: Ascii (ref s) => s . hash (hasher) , Encoding :: Unicode (ref s) => s . hash (hasher) , } } }
};
}
