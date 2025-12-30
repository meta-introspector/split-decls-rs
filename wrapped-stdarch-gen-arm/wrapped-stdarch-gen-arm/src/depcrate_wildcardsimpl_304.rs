// Generated macro for impl_304 (impl)
macro_rules! Depcrate_wildcardsimpl_304 {
() => {
// Module: crate::wildcards
// Provides: {"impl_304"}
// Dependencies: {}
impl Wildcard { pub fn is_nonpredicate_type (& self) -> bool { matches ! (self , Wildcard :: Type (..) | Wildcard :: NEONType (..) | Wildcard :: SVEType (..)) } pub fn get_typeset_index (& self) -> Option < usize > { match self { Wildcard :: Type (idx) | Wildcard :: NEONType (idx , ..) | Wildcard :: SVEType (idx , ..) => { Some (idx . unwrap_or (0)) } _ => None , } } }
};
}
