// Generated macro for get_max_level_inner (function)
macro_rules! Depcrate_level_filtersget_max_level_inner {
() => {
// Module: crate::level_filters
// Provides: {"get_max_level_inner"}
// Dependencies: {}
const fn get_max_level_inner () -> LevelFilter { if cfg ! (not (debug_assertions)) { if cfg ! (feature = "release_max_level_off") { LevelFilter :: OFF } else if cfg ! (feature = "release_max_level_error") { LevelFilter :: ERROR } else if cfg ! (feature = "release_max_level_warn") { LevelFilter :: WARN } else if cfg ! (feature = "release_max_level_info") { LevelFilter :: INFO } else if cfg ! (feature = "release_max_level_debug") { LevelFilter :: DEBUG } else { LevelFilter :: TRACE } } else if cfg ! (feature = "max_level_off") { LevelFilter :: OFF } else if cfg ! (feature = "max_level_error") { LevelFilter :: ERROR } else if cfg ! (feature = "max_level_warn") { LevelFilter :: WARN } else if cfg ! (feature = "max_level_info") { LevelFilter :: INFO } else if cfg ! (feature = "max_level_debug") { LevelFilter :: DEBUG } else { LevelFilter :: TRACE } }
};
}
