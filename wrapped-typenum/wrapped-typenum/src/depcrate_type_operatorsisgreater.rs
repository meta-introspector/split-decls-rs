// Generated macro for IsGreater (trait)
macro_rules! Depcrate_type_operatorsIsGreater {
() => {
// Module: crate::type_operators
// Provides: {"IsGreater"}
// Dependencies: {}
# [doc = " A **type operator** that returns `True` if `Self > Rhs`, otherwise returns `False`."] pub trait IsGreater < Rhs = Self > { # [doc = " The type representing either `True` or `False`"] type Output : Bit ; # [doc = " Method returning `True` or `False`."] # [allow (clippy :: wrong_self_convention)] fn is_greater (self , rhs : Rhs) -> Self :: Output ; }
};
}
