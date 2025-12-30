// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de , T : PartialEq > PartialEq < T > for Serde < T > where De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { fn eq (& self , other : & T) -> bool { self . 0 == * other } }
};
}
