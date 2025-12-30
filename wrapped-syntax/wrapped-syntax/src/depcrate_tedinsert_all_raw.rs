// Generated macro for insert_all_raw (function)
macro_rules! Depcrate_tedinsert_all_raw {
() => {
// Module: crate::ted
// Provides: {"insert_all_raw"}
// Dependencies: {}
pub fn insert_all_raw (position : Position , elements : Vec < SyntaxElement >) { let (parent , index) = match position . repr { PositionRepr :: FirstChild (parent) => (parent , 0) , PositionRepr :: After (child) => (child . parent () . unwrap () , child . index () + 1) , } ; parent . splice_children (index .. index , elements) ; }
};
}
