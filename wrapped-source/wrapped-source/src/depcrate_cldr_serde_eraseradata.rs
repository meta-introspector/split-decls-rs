// Generated macro for EraData (struct)
macro_rules! Depcrate_cldr_serde_erasEraData {
() => {
// Module: crate::cldr_serde::eras
// Provides: {"EraData"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , Clone)] pub (crate) struct EraData { # [serde (rename = "_start" , default , deserialize_with = "parse_era_start_date")] pub (crate) start : Option < EraStartDate > , # [serde (rename = "_end" , default , deserialize_with = "parse_era_start_date")] pub (crate) end : Option < EraStartDate > , # [serde (rename = "_code")] pub (crate) code : Option < String > , # [serde (rename = "_aliases")] pub (crate) aliases : Option < String > , # [doc = " EraYear::era_index"] # [serde (skip)] pub (crate) icu4x_era_index : Option < u8 > , }
};
}
