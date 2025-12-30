// Generated macro for impl_214 (impl)
macro_rules! Depcrate_filter_wrapimpl_214 {
() => {
// Module: crate::filter::wrap
// Provides: {"impl_214"}
// Dependencies: {}
impl < F , T , U > WrapSealed < T > for WrapFn < F > where F : Fn (T) -> U , T : Filter , U : Filter , { type Wrapped = U ; fn wrap (& self , filter : T) -> Self :: Wrapped { (self . func) (filter) } }
};
}
