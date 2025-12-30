// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < T > PartialOrd for ThinVec < T > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & ThinVec < T >) -> Option < Ordering > { self [..] . partial_cmp (& other [..]) } }
};
}
