// Generated macro for DefinitiveListTactic (enum)
macro_rules! Depcrate_config_listsDefinitiveListTactic {
() => {
// Module: crate::config::lists
// Provides: {"DefinitiveListTactic"}
// Dependencies: {}
# [doc = " The definitive formatting tactic for lists."] # [derive (Eq , PartialEq , Debug , Copy , Clone)] pub enum DefinitiveListTactic { Vertical , Horizontal , Mixed , # [doc = " Special case tactic for `format!()`, `write!()` style macros."] SpecialMacro (usize) , }
};
}
