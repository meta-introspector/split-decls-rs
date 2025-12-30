// Generated macro for impl_133 (impl)
macro_rules! Depcrate_intrinsicimpl_133 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_133"}
// Dependencies: {}
impl FromStr for Argument { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut it = s . splitn (2 , ':') . map (< str > :: trim) ; if let Some (mut lhs) = it . next () . map (| s | s . split_whitespace ()) { let lhs_len = lhs . clone () . count () ; match (lhs_len , lhs . next () , it . next ()) { (2 , Some ("mut") , Some (kind)) => Ok (Argument { name : lhs . next () . unwrap () . parse () ? , rw : AccessLevel :: RW , kind : kind . parse () ? , }) , (2 , Some (ident) , _) => Err (format ! ("invalid {ident:#?} keyword")) , (1 , Some (name) , Some (kind)) => Ok (Argument { name : name . parse () ? , rw : AccessLevel :: R , kind : kind . parse () ? , }) , _ => Err (format ! ("invalid argument `{s}` provided")) , } } else { Err (format ! ("invalid argument `{s}` provided")) } } }
};
}
