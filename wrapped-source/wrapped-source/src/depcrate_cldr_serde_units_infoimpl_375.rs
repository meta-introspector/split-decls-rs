// Generated macro for impl_375 (impl)
macro_rules! Depcrate_cldr_serde_units_infoimpl_375 {
() => {
// Module: crate::cldr_serde::units::info
// Provides: {"impl_375"}
// Dependencies: {}
impl Resource { # [doc = " Retrieves the unique identifier for a given unit name which is the unit's index in the list."] # [doc = ""] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(u16)` - The unique identifier for the unit if found."] # [doc = " * `Err(DataError)` - An error if the unit is not found or if the index is out of range for `u16`."] pub fn unit_id (& self , unit_name : & str) -> Result < UnitID , DataError > { CLDR_IDS_TRIE . get (unit_name) . ok_or_else (| | DataError :: custom ("Unit not found")) . and_then (| value | { UnitID :: try_from (value) . map_err (| _ | DataError :: custom ("Value out of range for u16")) }) } # [doc = " Constructs a map of unit names to their unique identifiers, which are the unit's indices in the list."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns a `DataError` if the index cannot be converted to `u16`."] pub fn unit_ids_map (& self) -> Result < BTreeMap < String , UnitID > , DataError > { CLDR_IDS_TRIE . iter () . map (| (unit_name , unit_id) | { UnitID :: try_from (unit_id) . map (| id | (unit_name . to_string () , id)) . map_err (| _ | DataError :: custom ("Value out of range for u16")) }) . collect :: < Result < BTreeMap < _ , _ > , _ > > () } }
};
}
