// Generated macro for impl_134 (impl)
macro_rules! Depcrate_valueimpl_134 {
() => {
// Module: crate::value
// Provides: {"impl_134"}
// Dependencies: {}
impl ValueSerializeVariant < ValueSerializeVec > { pub (crate) fn tuple (variant : & 'static str , len : usize) -> Self { Self { variant , inner : ValueSerializeVec { vec : Vec :: with_capacity (len) , } , } } }
};
}
