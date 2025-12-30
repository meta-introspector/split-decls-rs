// Generated macro for Numbers (struct)
macro_rules! Depcrate_cldr_serde_numbersNumbers {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"Numbers"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Numbers { # [serde (rename = "defaultNumberingSystem")] pub (crate) default_numbering_system : String , # [serde (rename = "minimumGroupingDigits")] # [serde (deserialize_with = "serde_aux::prelude::deserialize_number_from_string")] pub (crate) minimum_grouping_digits : u8 , # [serde (flatten)] pub (crate) numsys_data : NumberingSystemData , }
};
}
