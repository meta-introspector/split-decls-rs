// Generated macro for Tuplable (trait)
macro_rules! Depcrate_tuplableTuplable {
() => {
// Module: crate::tuplable
// Provides: {"Tuplable"}
// Dependencies: {}
# [doc = " A tuple-like [`Valuable`] sub-type."] # [doc = ""] # [doc = " Implemented by [`Valuable`] types that have a tuple-like shape. Fields are"] # [doc = " always unnamed. Values that implement `Tuplable` must return"] # [doc = " [`Value::Tuplable`] from their [`Valuable::as_value`] implementation."] # [doc = ""] # [doc = " It is uncommon for users to implement this type as the crate provides"] # [doc = " implementations of `Tuplable` for Rust tuples."] # [doc = ""] # [doc = " # Inspecting"] # [doc = ""] # [doc = " Inspecting fields contained by a `Tuplable` instance is done by visiting the"] # [doc = " tuple. When visiting a `Tuple`, the `visit_unnamed_fields()` method is"] # [doc = " called. When the tuple is statically defined, `visit_unnamed_fields()` is"] # [doc = " called once with the values of all the fields. A dynamic tuple"] # [doc = " implementation may call `visit_unnamed_fields()` multiple times."] pub trait Tuplable : Valuable { # [doc = " Returns the tuple's definition."] # [doc = ""] # [doc = " See [`TupleDef`] documentation for more details."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use valuable::{Tuplable, TupleDef};"] # [doc = ""] # [doc = " let tuple = (123, \"hello\");"] # [doc = ""] # [doc = " if let TupleDef::Static { fields, .. } = tuple.definition() {"] # [doc = "     assert_eq!(2, fields);"] # [doc = " }"] # [doc = " ```"] fn definition (& self) -> TupleDef ; }
};
}
