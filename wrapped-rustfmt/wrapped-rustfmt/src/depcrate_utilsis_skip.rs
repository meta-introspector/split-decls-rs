// Generated macro for is_skip (function)
macro_rules! Depcrate_utilsis_skip {
() => {
// Module: crate::utils
// Provides: {"is_skip"}
// Dependencies: {}
# [inline] fn is_skip (meta_item : & MetaItem) -> bool { match meta_item . kind { MetaItemKind :: Word => { let path_str = pprust :: path_to_string (& meta_item . path) ; path_str == skip_annotation () . as_str () || path_str == depr_skip_annotation () . as_str () } MetaItemKind :: List (ref l) => { meta_item . has_name (sym :: cfg_attr) && l . len () == 2 && is_skip_nested (& l [1]) } _ => false , } }
};
}
