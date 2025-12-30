// Generated macro for CalendarData (struct)
macro_rules! Depcrate_cldr_serde_erasCalendarData {
() => {
// Module: crate::cldr_serde::eras
// Provides: {"CalendarData"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , Clone)] pub (crate) struct CalendarData { # [serde (default)] pub (crate) eras : BTreeMap < String , EraData > , # [serde (rename = "inheritEras")] pub (crate) inherit_eras : Option < InheritEras > , }
};
}
