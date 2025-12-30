// Generated macro for rewrite_mod (function)
macro_rules! Depcrate_itemsrewrite_mod {
() => {
// Module: crate::items
// Provides: {"rewrite_mod"}
// Dependencies: {}
# [doc = " Rewrite an inline mod."] # [doc = " The given shape is used to format the mod's attributes."] pub (crate) fn rewrite_mod (context : & RewriteContext < '_ > , item : & ast :: Item , ident : Ident , attrs_shape : Shape ,) -> RewriteResult { let mut result = String :: with_capacity (32) ; result . push_str (& * format_visibility (context , & item . vis)) ; result . push_str ("mod ") ; result . push_str (rewrite_ident (context , ident)) ; result . push (';') ; rewrite_attrs (context , item , & result , attrs_shape) }
};
}
