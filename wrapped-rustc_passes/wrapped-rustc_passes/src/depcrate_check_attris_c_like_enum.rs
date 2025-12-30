// Generated macro for is_c_like_enum (function)
macro_rules! Depcrate_check_attris_c_like_enum {
() => {
// Module: crate::check_attr
// Provides: {"is_c_like_enum"}
// Dependencies: {}
fn is_c_like_enum (item : & Item < '_ >) -> bool { if let ItemKind :: Enum (_ , _ , ref def) = item . kind { for variant in def . variants { match variant . data { hir :: VariantData :: Unit (..) => { } _ => return false , } } true } else { false } }
};
}
