// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > UnwrapCapOverflow < T > for Option < T > { fn unwrap_cap_overflow (self) -> T { match self { Some (val) => val , None => capacity_overflow () , } } }
};
}
