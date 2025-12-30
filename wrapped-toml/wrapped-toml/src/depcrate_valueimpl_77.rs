// Generated macro for impl_77 (impl)
macro_rules! Depcrate_valueimpl_77 {
() => {
// Module: crate::value
// Provides: {"impl_77"}
// Dependencies: {}
impl < I > ops :: Index < I > for Value where I : Index , { type Output = Self ; fn index (& self , index : I) -> & Self { self . get (index) . expect ("index not found") } }
};
}
