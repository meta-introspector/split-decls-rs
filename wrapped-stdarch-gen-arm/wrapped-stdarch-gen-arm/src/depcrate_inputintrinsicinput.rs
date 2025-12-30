// Generated macro for IntrinsicInput (struct)
macro_rules! Depcrate_inputIntrinsicInput {
() => {
// Module: crate::input
// Provides: {"IntrinsicInput"}
// Dependencies: {}
# [derive (Debug , Clone , Default , Serialize , Deserialize)] pub struct IntrinsicInput { # [serde (default)] # [serde (deserialize_with = "validate_types")] pub types : Vec < InputSetEntry > , # [serde (flatten)] pub predication_methods : PredicationMethods , # [doc = " Generates a _n variant where the specified operand is a primitive type"] # [doc = " that requires conversion to an SVE one. The `{_n}` wildcard is required"] # [doc = " in the intrinsic's name, otherwise an error will be thrown."] # [serde (default)] pub n_variant_op : WildString , }
};
}
