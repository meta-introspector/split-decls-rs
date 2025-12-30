// Generated macro for impl_491 (impl)
macro_rules! Depcrate_ser_mapimpl_491 {
() => {
// Module: crate::ser::map
// Provides: {"impl_491"}
// Dependencies: {}
impl SerializeInlineTable { pub (crate) fn map (len : Option < usize >) -> Self { let mut items : crate :: table :: KeyValuePairs = Default :: default () ; let key = Default :: default () ; if let Some (len) = len { items . reserve (len) ; } Self { items , key } } }
};
}
