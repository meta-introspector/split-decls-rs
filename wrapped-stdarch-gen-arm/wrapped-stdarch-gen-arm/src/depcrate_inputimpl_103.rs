// Generated macro for impl_103 (impl)
macro_rules! Depcrate_inputimpl_103 {
() => {
// Module: crate::input
// Provides: {"impl_103"}
// Dependencies: {}
impl InputSet { pub fn get (& self , idx : usize) -> Option < & InputType > { self . 0 . get (idx) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } pub fn iter (& self) -> impl Iterator < Item = & InputType > + '_ { self . 0 . iter () } pub fn iter_mut (& mut self) -> impl Iterator < Item = & mut InputType > + '_ { self . 0 . iter_mut () } pub fn into_iter (self) -> impl Iterator < Item = InputType > + Clone { self . 0 . into_iter () } pub fn types_len (& self) -> usize { self . iter () . filter_map (| arg | arg . typekind ()) . count () } pub fn typekind (& self , idx : Option < usize >) -> Option < TypeKind > { let types_len = self . types_len () ; self . get (idx . unwrap_or (0)) . and_then (move | arg : & InputType | { if (idx . is_none () && types_len != 1) || (idx . is_some () && types_len == 1) { None } else { arg . typekind () . cloned () } }) } }
};
}
