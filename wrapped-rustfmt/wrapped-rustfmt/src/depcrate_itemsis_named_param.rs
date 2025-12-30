// Generated macro for is_named_param (function)
macro_rules! Depcrate_itemsis_named_param {
() => {
// Module: crate::items
// Provides: {"is_named_param"}
// Dependencies: {}
pub (crate) fn is_named_param (param : & ast :: Param) -> bool { if let ast :: PatKind :: Ident (_ , ident , _) = param . pat . kind { ident . name != symbol :: kw :: Empty } else { true } }
};
}
