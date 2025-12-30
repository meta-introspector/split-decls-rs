// Generated macro for enable (function)
macro_rules! Depcrate_globalenable {
() => {
// Module: crate::global
// Provides: {"enable"}
// Dependencies: {}
# [doc = " Unconditionally enables styling globally."] # [doc = ""] # [doc = " By default, styling is enabled based on [`Condition::DEFAULT`], which checks"] # [doc = " for operating system support."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::Paint;"] # [doc = ""] # [doc = " // With styling disabled, ANSI color codes are _not_ emitted."] # [doc = " yansi::disable();"] # [doc = " assert_eq!(\"go\".green().to_string(), \"go\".to_string());"] # [doc = ""] # [doc = " // Reenabling causes color code to be emitted."] # [doc = " yansi::enable();"] # [doc = " assert_ne!(\"go\".green().to_string(), \"go\".to_string());"] # [doc = " ```"] pub fn enable () { ENABLED . store (Condition :: ALWAYS) ; }
};
}
