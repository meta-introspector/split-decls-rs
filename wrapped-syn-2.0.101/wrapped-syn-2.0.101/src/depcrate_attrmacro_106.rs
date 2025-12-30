// Generated macro for macro_106 (macro)
macro_rules! Depcrate_attrmacro_106 {
() => {
// Module: crate::attr
// Provides: {"macro_106"}
// Dependencies: {}
ast_enum ! { # [doc = " Distinguishes between attributes that decorate an item and attributes"] # [doc = " that are contained within an item."] # [doc = ""] # [doc = " # Outer attributes"] # [doc = ""] # [doc = " - `#[repr(transparent)]`"] # [doc = " - `/// # Example`"] # [doc = " - `/** Please file an issue */`"] # [doc = ""] # [doc = " # Inner attributes"] # [doc = ""] # [doc = " - `#![feature(proc_macro)]`"] # [doc = " - `//! # Example`"] # [doc = " - `/*! Please file an issue */`"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum AttrStyle { Outer , Inner (Token ! [!]) , } }
};
}
