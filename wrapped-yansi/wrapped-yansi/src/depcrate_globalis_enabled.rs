// Generated macro for is_enabled (function)
macro_rules! Depcrate_globalis_enabled {
() => {
// Module: crate::global
// Provides: {"is_enabled"}
// Dependencies: {}
# [doc = " Returns `true` if styling is globally enabled and `false` otherwise."] # [doc = ""] # [doc = " By default, styling is enabled based on [`Condition::DEFAULT`], which checks"] # [doc = " for operating system support. It can be enabled and disabled on-the-fly with"] # [doc = " [`enable()`] and [`disable()`] and via a dynamic condition with"] # [doc = " [`whenever()`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " // Styling is enabled by default."] # [doc = " # yansi::enable();"] # [doc = " assert!(yansi::is_enabled());"] # [doc = ""] # [doc = " // Disable it with `Painted::disable()`."] # [doc = " yansi::disable();"] # [doc = " assert!(!yansi::is_enabled());"] # [doc = ""] # [doc = " // Reenable with `Painted::enable()`."] # [doc = " yansi::enable();"] # [doc = " assert!(yansi::is_enabled());"] # [doc = " ```"] pub fn is_enabled () -> bool { ENABLED . read () }
};
}
