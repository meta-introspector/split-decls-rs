// Generated macro for Constraint (enum)
macro_rules! Depcrate_intrinsicConstraint {
() => {
// Module: crate::intrinsic
// Provides: {"Constraint"}
// Dependencies: {}
# [doc = " Function constraints"] # [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged)] pub enum Constraint { # [doc = " Asserts that the given variable equals to any of the given integer values"] AnyI32 { variable : String , any_values : Vec < i32 > , } , # [doc = " WildString version of RangeI32. If the string values given for the range"] # [doc = " are valid, this gets built into a RangeI32."] RangeWildstring { variable : String , range : (WildString , WildString) , } , # [doc = " Asserts that the given variable's value falls in the specified range"] RangeI32 { variable : String , range : SizeMatchable < RangeInclusive < i32 > > , } , # [doc = " Asserts that the number of elements/lanes does not exceed the 2048-bit SVE constraint"] SVEMaxElems { variable : String , sve_max_elems_type : TypeKind , } , # [doc = " Asserts that the number of elements/lanes does not exceed the 128-bit register constraint"] VecMaxElems { variable : String , vec_max_elems_type : TypeKind , } , }
};
}
