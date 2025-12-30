// Generated macro for level_to_log (macro)
macro_rules! Depcrate_macroslevel_to_log {
() => {
// Module: crate::macros
// Provides: {"level_to_log"}
// Dependencies: {}
# [cfg (feature = "log")] # [doc (hidden)] # [macro_export] macro_rules ! level_to_log { ($ level : expr) => { match $ level { $ crate :: Level :: ERROR => $ crate :: log :: Level :: Error , $ crate :: Level :: WARN => $ crate :: log :: Level :: Warn , $ crate :: Level :: INFO => $ crate :: log :: Level :: Info , $ crate :: Level :: DEBUG => $ crate :: log :: Level :: Debug , _ => $ crate :: log :: Level :: Trace , } } ; }
};
}
