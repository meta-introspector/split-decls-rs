// Generated macro for format_visibility (function)
macro_rules! Depcrate_utilsformat_visibility {
() => {
// Module: crate::utils
// Provides: {"format_visibility"}
// Dependencies: {}
pub (crate) fn format_visibility (context : & RewriteContext < '_ > , vis : & Visibility ,) -> Cow < 'static , str > { match vis . kind { VisibilityKind :: Public => Cow :: from ("pub ") , VisibilityKind :: Inherited => Cow :: from ("") , VisibilityKind :: Restricted { ref path , .. } => { let Path { ref segments , .. } = * * path ; let mut segments_iter = segments . iter () . map (| seg | rewrite_ident (context , seg . ident)) ; if path . is_global () { segments_iter . next () . expect ("Non-global path in pub(restricted)?") ; } let is_keyword = | s : & str | s == "crate" || s == "self" || s == "super" ; let path = segments_iter . collect :: < Vec < _ > > () . join ("::") ; let in_str = if is_keyword (& path) { "" } else { "in " } ; Cow :: from (format ! ("pub({in_str}{path}) ")) } } }
};
}
