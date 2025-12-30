// Generated macro for syntax_violation_enum (macro)
macro_rules! Depcrate_parsersyntax_violation_enum {
() => {
// Module: crate::parser
// Provides: {"syntax_violation_enum"}
// Dependencies: {}
macro_rules ! syntax_violation_enum { ($ ($ name : ident => $ description : literal ,) +) => { # [doc = " Non-fatal syntax violations that can occur during parsing."] # [doc = ""] # [doc = " This may be extended in the future so exhaustive matching is"] # [doc = " forbidden."] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [non_exhaustive] pub enum SyntaxViolation { $ (# [doc = " ```text"] # [doc = $ description] # [doc = " ```"] $ name ,) + } impl SyntaxViolation { pub fn description (& self) -> &'static str { match * self { $ (SyntaxViolation ::$ name => $ description ,) + } } } } }
};
}
