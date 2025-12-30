// Generated macro for impl_225 (impl)
macro_rules! Depcrateimpl_225 {
() => {
// Module: crate
// Provides: {"impl_225"}
// Dependencies: {}
impl < T , C > PartialEq < T > for Entry < '_ , T , C > where T : PartialEq < T > , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { self . value () . eq (other) } }
};
}
