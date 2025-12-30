// Generated macro for Signature (struct)
macro_rules! Depcrate_intrinsicSignature {
() => {
// Module: crate::intrinsic
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Function signature"] # [derive (Debug , Clone , Default , Serialize , Deserialize)] pub struct Signature { # [doc = " Function name"] pub name : WildString , # [doc = " List of function arguments, leave unset or empty for no arguments"] pub arguments : Vec < Argument > , # [doc = " Function return type, leave unset for void"] pub return_type : Option < TypeKind > , # [doc = " For some neon intrinsics we want to modify the suffix of the function name"] pub suffix_type : Option < SuffixKind > , # [doc = " List of static definitions, leave unset of empty if not required"] # [serde (default)] pub static_defs : Vec < StaticDefinition > , # [doc = " **Internal use only.**"] # [doc = " Condition for which the ultimate function is specific to predicates."] # [serde (skip)] pub is_predicate_specific : bool , # [doc = " **Internal use only.**"] # [doc = " Setting this property will trigger the signature builder to convert any `svbool*_t` to `svbool_t` in the input and output."] # [serde (skip)] pub predicate_needs_conversion : bool , }
};
}
