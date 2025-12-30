// Generated macro for impl_460 (impl)
macro_rules! Depcrate_ser_arrayimpl_460 {
() => {
// Module: crate::ser::array
// Provides: {"impl_460"}
// Dependencies: {}
impl SerializeTupleVariant { pub (crate) fn tuple (variant : & 'static str , len : usize) -> Self { Self { variant , inner : SerializeValueArray :: seq (Some (len)) , } } }
};
}
