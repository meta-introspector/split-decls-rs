// Generated macro for level_enabled (macro)
macro_rules! Depcrate_macroslevel_enabled {
() => {
// Module: crate::macros
// Provides: {"level_enabled"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! level_enabled { ($ lvl : expr) => { $ lvl <= $ crate :: level_filters :: STATIC_MAX_LEVEL && $ lvl <= $ crate :: level_filters :: LevelFilter :: current () } ; }
};
}
