// Generated macro for div_floor (macro)
macro_rules! Depcrate_internal_macrosdiv_floor {
() => {
// Module: crate::internal_macros
// Provides: {"div_floor"}
// Dependencies: {}
# [doc = " Division of integers, rounding the resulting value towards negative infinity."] macro_rules ! div_floor { ($ self : expr , $ rhs : expr) => { match ($ self , $ rhs) { (this , rhs) => { let d = this / rhs ; let r = this % rhs ; let correction = (this ^ rhs) >> ($ crate :: size_of_val (& this) * 8 - 1) ; if r != 0 { d + correction } else { d } } } } ; }
};
}
