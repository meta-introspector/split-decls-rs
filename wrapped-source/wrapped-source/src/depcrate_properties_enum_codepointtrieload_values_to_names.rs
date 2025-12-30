// Generated macro for load_values_to_names (function)
macro_rules! Depcrate_properties_enum_codepointtrieload_values_to_names {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"load_values_to_names"}
// Dependencies: {}
# [doc = " Load the mapping from property values to their names"] fn load_values_to_names (data : & super :: uprops_serde :: enumerated :: EnumeratedPropertyMap , is_short : bool ,) -> Result < BTreeMap < u16 , & str > , DataError > { let mut map : BTreeMap < _ , & str > = BTreeMap :: new () ; for value in & data . values { let discr = u16 :: try_from (value . discr) . map_err (| _ | DataError :: custom ("Found value larger than u16 for property")) ? ; if is_short { if let Some (ref short) = value . short { map . insert (discr , short) ; } } else { map . insert (discr , & value . long) ; } } Ok (map) }
};
}
