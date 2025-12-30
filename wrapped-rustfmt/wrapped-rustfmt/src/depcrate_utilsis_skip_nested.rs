// Generated macro for is_skip_nested (function)
macro_rules! Depcrate_utilsis_skip_nested {
() => {
// Module: crate::utils
// Provides: {"is_skip_nested"}
// Dependencies: {}
# [inline] fn is_skip_nested (meta_item : & MetaItemInner) -> bool { match meta_item { MetaItemInner :: MetaItem (ref mi) => is_skip (mi) , MetaItemInner :: Lit (_) => false , } }
};
}
