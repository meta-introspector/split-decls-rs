// Generated macro for is_same_visibility (function)
macro_rules! Depcrate_utilsis_same_visibility {
() => {
// Module: crate::utils
// Provides: {"is_same_visibility"}
// Dependencies: {}
pub (crate) fn is_same_visibility (a : & Visibility , b : & Visibility) -> bool { match (& a . kind , & b . kind) { (VisibilityKind :: Restricted { path : p , .. } , VisibilityKind :: Restricted { path : q , .. } ,) => pprust :: path_to_string (p) == pprust :: path_to_string (q) , (VisibilityKind :: Public , VisibilityKind :: Public) | (VisibilityKind :: Inherited , VisibilityKind :: Inherited) => true , _ => false , } }
};
}
