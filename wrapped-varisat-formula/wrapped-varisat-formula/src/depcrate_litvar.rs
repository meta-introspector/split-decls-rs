// Generated macro for Var (struct)
macro_rules! Depcrate_litVar {
() => {
// Module: crate::lit
// Provides: {"Var"}
// Dependencies: {}
# [doc = " A boolean variable."] # [doc = ""] # [doc = " A boolean value is represented by an index. Internally these are 0-based, i.e. the first"] # [doc = " variable has the index 0. For user IO a 1-based index is used, to allow denoting negated"] # [doc = " variables using negative integers. This convention is also used in the DIMACS CNF format."] # [doc = ""] # [doc = " Creating a variable with an index larger than `Var::max_var().index()` is unsupported. This"] # [doc = " might panic or be interpreted as a different variable."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct Var { index : LitIdx , }
};
}
