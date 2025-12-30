// Generated macro for is_mod_decl (function)
macro_rules! Depcrate_itemsis_mod_decl {
() => {
// Module: crate::items
// Provides: {"is_mod_decl"}
// Dependencies: {}
# [doc = " Returns `true` for `mod foo;`, false for `mod foo { .. }`."] pub (crate) fn is_mod_decl (item : & ast :: Item) -> bool { ! matches ! (item . kind , ast :: ItemKind :: Mod (_ , _ , ast :: ModKind :: Loaded (_ , ast :: Inline :: Yes , _ , _))) }
};
}
