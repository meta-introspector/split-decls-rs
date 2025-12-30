// Generated macro for get_prop_values_map (function)
macro_rules! Depcrate_properties_enum_codepointtrieget_prop_values_map {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"get_prop_values_map"}
// Dependencies: {}
fn get_prop_values_map < F > (values : & [super :: uprops_serde :: PropertyValue] , transform_u32 : F ,) -> Result < PropertyValueNameToEnumMap < 'static > , DataError > where F : Fn (u32) -> Result < u16 , DataError > , { let mut map = BTreeMap :: new () ; for value in values { let discr = transform_u32 (value . discr) ? as usize ; map . insert (value . long . as_bytes () , discr) ; if let Some (ref short) = value . short { map . insert (short . as_bytes () , discr) ; } for alias in & value . aliases { map . insert (alias . as_bytes () , discr) ; } } Ok (PropertyValueNameToEnumMap { map : ZeroTrieSimpleAscii :: from_iter (map) . convert_store () , }) }
};
}
