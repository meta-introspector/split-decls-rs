// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < T , E > UnwrapCapOverflow < T > for Result < T , E > { fn unwrap_cap_overflow (self) -> T { match self { Ok (val) => val , Err (_) => capacity_overflow () , } } }
};
}
