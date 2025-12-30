// Generated macro for RhsTactics (enum)
macro_rules! Depcrate_exprRhsTactics {
() => {
// Module: crate::expr
// Provides: {"RhsTactics"}
// Dependencies: {}
# [doc = " Controls where to put the rhs."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub (crate) enum RhsTactics { # [doc = " Use heuristics."] Default , # [doc = " Put the rhs on the next line if it uses multiple line, without extra indentation."] ForceNextLineWithoutIndent , # [doc = " Allow overflowing max width if neither `Default` nor `ForceNextLineWithoutIndent`"] # [doc = " did not work."] AllowOverflow , }
};
}
