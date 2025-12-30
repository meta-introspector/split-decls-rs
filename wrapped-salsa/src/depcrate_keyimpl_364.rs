// Generated macro for impl_364 (impl)
macro_rules! Depcrate_keyimpl_364 {
() => {
// Module: crate::key
// Provides: {"impl_364"}
// Dependencies: {}
impl fmt :: Debug for DatabaseKeyIndex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { crate :: attach :: with_attached_database (| db | { let ingredient = db . zalsa () . lookup_ingredient (self . ingredient_index ()) ; ingredient . fmt_index (self . key_index () , f) }) . unwrap_or_else (| | { f . debug_tuple ("DatabaseKeyIndex") . field (& self . ingredient_index ()) . field (& self . key_index ()) . finish () }) } }
};
}
