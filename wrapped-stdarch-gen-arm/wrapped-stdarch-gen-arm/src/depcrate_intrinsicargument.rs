// Generated macro for Argument (struct)
macro_rules! Depcrate_intrinsicArgument {
() => {
// Module: crate::intrinsic
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Function signature argument."] # [doc = ""] # [doc = " Prepend the `mut` keyword for a mutable argument. Separate argument name"] # [doc = " and type with a semicolon `:`. Usage examples:"] # [doc = " - Mutable argument: `mut arg1: *u64`"] # [doc = " - Immutable argument: `arg2: u32`"] # [derive (Debug , Clone , SerializeDisplay , DeserializeFromStr)] pub struct Argument { # [doc = " Argument name"] pub name : WildString , # [doc = " Mutability level"] pub rw : AccessLevel , # [doc = " Argument type"] pub kind : TypeKind , }
};
}
