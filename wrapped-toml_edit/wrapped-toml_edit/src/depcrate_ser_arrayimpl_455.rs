// Generated macro for impl_455 (impl)
macro_rules! Depcrate_ser_arrayimpl_455 {
() => {
// Module: crate::ser::array
// Provides: {"impl_455"}
// Dependencies: {}
impl SerializeValueArray { pub (crate) fn seq (len : Option < usize >) -> Self { let mut values = Vec :: new () ; if let Some (len) = len { values . reserve (len) ; } Self { values } } }
};
}
