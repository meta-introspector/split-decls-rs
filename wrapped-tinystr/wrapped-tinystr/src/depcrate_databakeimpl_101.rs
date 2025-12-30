// Generated macro for impl_101 (impl)
macro_rules! Depcrate_databakeimpl_101 {
() => {
// Module: crate::databake
// Provides: {"impl_101"}
// Dependencies: {}
impl < const N : usize > Bake for TinyAsciiStr < N > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("tinystr") ; let string = self . as_str () ; quote ! { tinystr :: tinystr ! (# N , # string) } } }
};
}
