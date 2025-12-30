// Generated macro for impl_204 (impl)
macro_rules! Depcrate_dataimpl_204 {
() => {
// Module: crate::data
// Provides: {"impl_204"}
// Dependencies: {}
impl < 'a > Iterator for Members < 'a > { type Item = Member ; fn next (& mut self) -> Option < Self :: Item > { let field = self . fields . next () ? ; let member = match & field . ident { Some (ident) => Member :: Named (ident . clone ()) , None => { # [cfg (all (feature = "parsing" , feature = "printing"))] let span = crate :: spanned :: Spanned :: span (& field . ty) ; # [cfg (not (all (feature = "parsing" , feature = "printing")))] let span = proc_macro2 :: Span :: call_site () ; Member :: Unnamed (Index { index : self . index , span , }) } } ; self . index += 1 ; Some (member) } }
};
}
