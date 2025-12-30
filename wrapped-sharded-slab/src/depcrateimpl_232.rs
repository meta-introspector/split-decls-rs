// Generated macro for impl_232 (impl)
macro_rules! Depcrateimpl_232 {
() => {
// Module: crate
// Provides: {"impl_232"}
// Dependencies: {}
impl < T , C > PartialEq < T > for OwnedEntry < T , C > where T : PartialEq < T > , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
};
}
