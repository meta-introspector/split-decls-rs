// Generated macro for impl_1321 (impl)
macro_rules! Depcrate_spannedimpl_1321 {
() => {
// Module: crate::spanned
// Provides: {"impl_1321"}
// Dependencies: {}
impl Spanned for ast :: GenericParam { fn span (& self) -> Span { let lo = match self . kind { _ if ! self . attrs . is_empty () => self . attrs [0] . span . lo () , ast :: GenericParamKind :: Const { span , .. } => span . lo () , _ => self . ident . span . lo () , } ; let hi = if self . bounds . is_empty () { self . ident . span . hi () } else { self . bounds . last () . unwrap () . span () . hi () } ; let ty_hi = if let ast :: GenericParamKind :: Type { default : Some (ref ty) , } | ast :: GenericParamKind :: Const { ref ty , .. } = self . kind { ty . span () . hi () } else { hi } ; mk_sp (lo , max (hi , ty_hi)) } }
};
}
