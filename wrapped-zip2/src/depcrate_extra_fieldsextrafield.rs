// Generated macro for ExtraField (enum)
macro_rules! Depcrate_extra_fieldsExtraField {
() => {
// Module: crate::extra_fields
// Provides: {"ExtraField"}
// Dependencies: {}
# [doc = " contains one extra field"] # [derive (Debug , Clone)] pub enum ExtraField { # [doc = " NTFS extra field"] Ntfs (Ntfs) , # [doc = " extended timestamp, as described in <https://libzip.org/specifications/extrafld.txt>"] ExtendedTimestamp (ExtendedTimestamp) , }
};
}
