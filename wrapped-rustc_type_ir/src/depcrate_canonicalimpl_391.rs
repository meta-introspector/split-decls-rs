// Generated macro for impl_391 (impl)
macro_rules! Depcrate_canonicalimpl_391 {
() => {
// Module: crate::canonical
// Provides: {"impl_391"}
// Dependencies: {}
impl < I : Interner > Index < ty :: BoundVar > for CanonicalVarValues < I > { type Output = I :: GenericArg ; fn index (& self , value : ty :: BoundVar) -> & I :: GenericArg { & self . var_values . as_slice () [value . as_usize ()] } }
};
}
