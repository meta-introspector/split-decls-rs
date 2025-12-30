// Generated macro for Lit (struct)
macro_rules! Depcrate_litLit {
() => {
// Module: crate::lit
// Provides: {"Lit"}
// Dependencies: {}
# [doc = " A boolean literal."] # [doc = ""] # [doc = " A literal is a variable or the negation of a variable."] # [doc = ""] # [doc = " Conceptually a literal consists of a `Var` and a `bool` indicating whether the literal"] # [doc = " represents the variable (positive literal) or its negation (negative literal)."] # [doc = ""] # [doc = " Internally a literal is represented as an integer that is two times the index of its variable"] # [doc = " when it is positive or one more when it is negative. This integer is called the `code` of the"] # [doc = " literal."] # [doc = ""] # [doc = " The restriction on the range of allowed indices for `Var` also applies to `Lit`."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct Lit { code : LitIdx , }
};
}
