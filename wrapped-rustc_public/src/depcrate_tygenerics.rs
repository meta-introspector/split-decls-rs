// Generated macro for Generics (struct)
macro_rules! Depcrate_tyGenerics {
() => {
// Module: crate::ty
// Provides: {"Generics"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Generics { pub parent : Option < GenericDef > , pub parent_count : usize , pub params : Vec < GenericParamDef > , pub param_def_id_to_index : Vec < (GenericDef , u32) > , pub has_self : bool , pub has_late_bound_regions : Option < Span > , }
};
}
