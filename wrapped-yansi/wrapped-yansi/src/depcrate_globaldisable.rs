// Generated macro for disable (function)
macro_rules! Depcrate_globaldisable {
() => {
// Module: crate::global
// Provides: {"disable"}
// Dependencies: {}
# [doc = " Unconditionally disables styling globally."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::Paint;"] # [doc = ""] # [doc = " // With styling enabled, ANSI color codes are emitted, thus `ne`."] # [doc = " assert_ne!(\"go\".green().to_string(), \"go\".to_string());"] # [doc = ""] # [doc = " // With styling disabled, ANSI color codes are _not_ emitted."] # [doc = " yansi::disable();"] # [doc = " assert_eq!(\"go\".green().to_string(), \"go\".to_string());"] # [doc = " ```"] pub fn disable () { ENABLED . store (Condition :: NEVER) ; }
};
}
