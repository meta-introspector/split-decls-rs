// Generated macro for STATIC_MAX_LEVEL (const)
macro_rules! Depcrate_level_filtersSTATIC_MAX_LEVEL {
() => {
// Module: crate::level_filters
// Provides: {"STATIC_MAX_LEVEL"}
// Dependencies: {}
# [doc = " The statically configured maximum trace level."] # [doc = ""] # [doc = " See the [module-level documentation] for information on how to configure"] # [doc = " this."] # [doc = ""] # [doc = " This value is checked by the `event!` and `span!` macros. Code that"] # [doc = " manually constructs events or spans via the `Event::record` function or"] # [doc = " `Span` constructors should compare the level against this value to"] # [doc = " determine if those spans or events are enabled."] # [doc = ""] # [doc = " [module-level documentation]: self#compile-time-filters"] pub const STATIC_MAX_LEVEL : LevelFilter = get_max_level_inner () ;
};
}
