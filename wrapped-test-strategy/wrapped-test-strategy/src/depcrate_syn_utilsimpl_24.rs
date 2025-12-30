// Generated macro for impl_24 (impl)
macro_rules! Depcrate_syn_utilsimpl_24 {
() => {
// Module: crate::syn_utils
// Provides: {"impl_24"}
// Dependencies: {}
impl RawFieldKey { fn from_ident (ident : & Ident) -> Self { Self :: Named (ident . unraw () . to_string ()) } fn from_field (idx : usize , field : & Field) -> Self { if let Some (ident) = & field . ident { Self :: from_ident (ident) } else { Self :: Unnamed (idx) } } fn try_from_token (token : & TokenTree) -> Option < Self > { match token { TokenTree :: Ident (ident) => Some (Self :: from_ident (ident)) , TokenTree :: Literal (token) => { if let Lit :: Int (lit) = Lit :: new (token . clone ()) { if lit . suffix () . is_empty () { if let Ok (idx) = lit . base10_parse () { return Some (Self :: Unnamed (idx)) ; } } } None } _ => None , } } fn to_valid_ident (& self) -> Option < Ident > { match self { Self :: Named (name) => to_valid_ident (name) . ok () , Self :: Unnamed (..) => None , } } }
};
}
