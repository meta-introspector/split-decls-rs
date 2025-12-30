// Generated macro for Category (enum)
macro_rules! Depcrate_builder_expr_categoryCategory {
() => {
// Module: crate::builder::expr::category
// Provides: {"Category"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) enum Category { # [doc = " An assignable memory location like `x`, `x.f`, `foo()[3]`, that"] # [doc = " sort of thing. Something that could appear on the LHS of an `=`"] # [doc = " sign."] Place , # [doc = " A literal like `23` or `\"foo\"`. Does not include constant"] # [doc = " expressions like `3 + 5`."] Constant , # [doc = " Something that generates a new value at runtime, like `x + y`"] # [doc = " or `foo()`."] Rvalue (RvalueFunc) , }
};
}
