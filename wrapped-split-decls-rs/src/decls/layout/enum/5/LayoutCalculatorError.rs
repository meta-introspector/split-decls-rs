use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum LayoutCalculatorError < F > { # [doc = " An unsized type was found in a location where a sized type was expected."] # [doc = ""] # [doc = " This is not always a compile error, for example if there is a `[T]: Sized`"] # [doc = " bound in a where clause."] # [doc = ""] # [doc = " Contains the field that was unexpectedly unsized."] UnexpectedUnsized (F) , # [doc = " A type was too large for the target platform."] SizeOverflow , # [doc = " A union had no fields."] EmptyUnion , # [doc = " The fields or variants have irreconcilable reprs"] ReprConflict , # [doc = " The length of an SIMD type is zero"] ZeroLengthSimdType , # [doc = " The length of an SIMD type exceeds the maximum number of lanes"] OversizedSimdType { max_lanes : u64 } , # [doc = " An element type of an SIMD type isn't a primitive"] NonPrimitiveSimdType (F) , }
}