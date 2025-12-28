macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! STATIC_MAX_LEVEL {
    () => {
        deps!();
        # [doc = " The statically configured maximum trace level."] # [doc = ""] # [doc = " See the [module-level documentation] for information on how to configure"] # [doc = " this."] # [doc = ""] # [doc = " This value is checked by the `event!` and `span!` macros. Code that"] # [doc = " manually constructs events or spans via the `Event::record` function or"] # [doc = " `Span` constructors should compare the level against this value to"] # [doc = " determine if those spans or events are enabled."] # [doc = ""] # [doc = " [module-level documentation]: self#compile-time-filters"] pub const STATIC_MAX_LEVEL : LevelFilter = get_max_level_inner () ;
    };
}

STATIC_MAX_LEVEL!()