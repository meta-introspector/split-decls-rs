// Generated macro for macro_23 (macro)
macro_rules! Depcrate_attrmacro_23 {
() => {
// Module: crate::attr
// Provides: {"macro_23"}
// Dependencies: {}
ast_enum ! { # [doc = " Distinguishes between Attributes that decorate items and Attributes that"] # [doc = " are contained as statements within items. These two cases need to be"] # [doc = " distinguished for pretty-printing."] # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum AttrStyle { # [doc = " Attribute of the form `#[...]`."] Outer , # [doc = " Attribute of the form `#![...]`."] Inner (tokens :: Bang) , } }
};
}
