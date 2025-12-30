// Generated macro for impl_237 (impl)
macro_rules! Depcrate_metadataimpl_237 {
() => {
// Module: crate::metadata
// Provides: {"impl_237"}
// Dependencies: {}
impl Level { # [doc = " The \"error\" level."] # [doc = ""] # [doc = " Designates very serious errors."] pub const ERROR : Level = Level (LevelInner :: Error) ; # [doc = " The \"warn\" level."] # [doc = ""] # [doc = " Designates hazardous situations."] pub const WARN : Level = Level (LevelInner :: Warn) ; # [doc = " The \"info\" level."] # [doc = ""] # [doc = " Designates useful information."] pub const INFO : Level = Level (LevelInner :: Info) ; # [doc = " The \"debug\" level."] # [doc = ""] # [doc = " Designates lower priority information."] pub const DEBUG : Level = Level (LevelInner :: Debug) ; # [doc = " The \"trace\" level."] # [doc = ""] # [doc = " Designates very low priority, often extremely verbose, information."] pub const TRACE : Level = Level (LevelInner :: Trace) ; # [doc = " Returns the string representation of the `Level`."] # [doc = ""] # [doc = " This returns the same string as the `fmt::Display` implementation."] pub fn as_str (& self) -> & 'static str { match * self { Level :: TRACE => "TRACE" , Level :: DEBUG => "DEBUG" , Level :: INFO => "INFO" , Level :: WARN => "WARN" , Level :: ERROR => "ERROR" , } } }
};
}
