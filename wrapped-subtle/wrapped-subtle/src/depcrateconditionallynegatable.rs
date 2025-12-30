// Generated macro for ConditionallyNegatable (trait)
macro_rules! DepcrateConditionallyNegatable {
() => {
// Module: crate
// Provides: {"ConditionallyNegatable"}
// Dependencies: {}
# [doc = " A type which can be conditionally negated in constant time."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " A generic implementation of `ConditionallyNegatable` is provided"] # [doc = " for types `T` which are `ConditionallySelectable` and have `Neg`"] # [doc = " implemented on `&T`."] # [allow (unused_attributes)] pub trait ConditionallyNegatable { # [doc = " Negate `self` if `choice == Choice(1)`; otherwise, leave it"] # [doc = " unchanged."] # [doc = ""] # [doc = " This function should execute in constant time."] # [inline] # [allow (unused_attributes)] fn conditional_negate (& mut self , choice : Choice) ; }
};
}
