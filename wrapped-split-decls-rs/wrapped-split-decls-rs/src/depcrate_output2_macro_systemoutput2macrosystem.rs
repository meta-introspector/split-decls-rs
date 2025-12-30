// Generated macro for Output2MacroSystem (struct)
macro_rules! Depcrate_output2_macro_systemOutput2MacroSystem {
() => {
// Module: crate::output2_macro_system
// Provides: {"Output2MacroSystem"}
// Dependencies: {}
# [doc = " Import all output2 declarations as callable macros"] # [doc = " Creates a Lisp-like runtime system for code generation"] # [derive (Debug)] pub struct Output2MacroSystem { # [doc = " All available macros from output2 declarations"] pub macros : HashMap < String , MacroDeclaration > , # [doc = " Runtime interpreter for macro calls"] pub interpreter : LispInterpreter , }
};
}
