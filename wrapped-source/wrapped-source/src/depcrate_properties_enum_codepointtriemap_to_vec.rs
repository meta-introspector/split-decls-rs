// Generated macro for map_to_vec (function)
macro_rules! Depcrate_properties_enum_codepointtriemap_to_vec {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"map_to_vec"}
// Dependencies: {}
# [doc = " Convert a map from property values to their names into"] # [doc = " a linear map where each index represents a property value"] fn map_to_vec < 'a > (map : & 'a BTreeMap < u16 , & 'a str > , prop_name : & str ,) -> Result < Vec < & 'a str > , DataError > { let first = if let Some ((& first , _)) = map . iter () . next () { if first > 0 { return Err (DataError :: custom ("Property has nonzero starting discriminant, perhaps consider \
                 storing its names as a sparse map or by specializing this error" ,) . with_display_context (& format ! ("Property: {prop_name}, discr: {first}"))) ; } first } else { return Err (DataError :: custom ("Property has no values!") . with_display_context (prop_name)) ; } ; let last = if let Some ((& last , _)) = map . iter () . next_back () { let range = usize :: from (1 + last - first) ; let count = map . len () ; let gaps = range - count ; if gaps > 0 { return Err (DataError :: custom ("Property has more than 0 gaps, \
                perhaps consider storing its names in a sparse map or by specializing this error") . with_display_context (& format ! ("Property: {prop_name}, discriminant range: {first}..{last}, discriminant count: {count}"))) ; } last } else { return Err (DataError :: custom ("Property has no values!") . with_display_context (prop_name)) ; } ; let mut v = Vec :: new () ; for i in 0 ..= last { if let Some (& val) = map . get (& i) { v . push (val) } else { v . push ("") } } Ok (v) }
};
}
