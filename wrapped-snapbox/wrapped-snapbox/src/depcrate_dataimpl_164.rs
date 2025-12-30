// Generated macro for impl_164 (impl)
macro_rules! Depcrate_dataimpl_164 {
() => {
// Module: crate::data
// Provides: {"impl_164"}
// Dependencies: {}
impl PartialEq for Data { fn eq (& self , other : & Data) -> bool { match (& self . inner , & other . inner) { (DataInner :: Error (left) , DataInner :: Error (right)) => left == right , (DataInner :: Binary (left) , DataInner :: Binary (right)) => left == right , (DataInner :: Text (left) , DataInner :: Text (right)) => left == right , # [cfg (feature = "json")] (DataInner :: Json (left) , DataInner :: Json (right)) => left == right , # [cfg (feature = "json")] (DataInner :: JsonLines (left) , DataInner :: JsonLines (right)) => left == right , # [cfg (feature = "term-svg")] (DataInner :: TermSvg (left) , DataInner :: TermSvg (right)) => { let left = term_svg_body (left . as_str ()) . unwrap_or (left . as_str ()) ; let right = term_svg_body (right . as_str ()) . unwrap_or (right . as_str ()) ; left == right } (_ , _) => false , } } }
};
}
