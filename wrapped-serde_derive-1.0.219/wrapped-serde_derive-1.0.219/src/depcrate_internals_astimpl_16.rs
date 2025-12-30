// Generated macro for impl_16 (impl)
macro_rules! Depcrate_internals_astimpl_16 {
() => {
// Module: crate::internals::ast
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > Data < 'a > { pub fn all_fields (& 'a self) -> Box < dyn Iterator < Item = & 'a Field < 'a > > + 'a > { match self { Data :: Enum (variants) => { Box :: new (variants . iter () . flat_map (| variant | variant . fields . iter ())) } Data :: Struct (_ , fields) => Box :: new (fields . iter ()) , } } pub fn has_getter (& self) -> bool { self . all_fields () . any (| f | f . attrs . getter () . is_some ()) } }
};
}
