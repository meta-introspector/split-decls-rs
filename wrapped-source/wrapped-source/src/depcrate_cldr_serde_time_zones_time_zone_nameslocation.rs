// Generated macro for Location (struct)
macro_rules! Depcrate_cldr_serde_time_zones_time_zone_namesLocation {
() => {
// Module: crate::cldr_serde::time_zones::time_zone_names
// Provides: {"Location"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] # [serde (deny_unknown_fields)] pub (crate) struct Location { pub (crate) long : Option < ZoneFormat > , pub (crate) short : Option < ZoneFormat > , # [serde (rename = "exemplarCity")] pub (crate) exemplar_city : Option < String > , # [serde (rename = "exemplarCity-alt-secondary")] pub (crate) _exemplar_city_alt_secondary : Option < String > , # [serde (rename = "_type")] pub (crate) _ty : Option < String > , }
};
}
