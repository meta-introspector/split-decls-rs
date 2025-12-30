// Generated macro for DateTimeFormats (struct)
macro_rules! Depcrate_cldr_serde_caDateTimeFormats {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"DateTimeFormats"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , Clone)] pub (crate) struct DateTimeFormats { pub (crate) full : LengthPattern , pub (crate) long : LengthPattern , pub (crate) medium : LengthPattern , pub (crate) short : LengthPattern , # [serde (rename = "availableFormats")] pub (crate) available_formats : AvailableFormats , }
};
}
