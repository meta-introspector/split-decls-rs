// Generated macro for PosixTzString (struct)
macro_rules! Depcrate_data_posixPosixTzString {
() => {
// Module: crate::data::posix
// Provides: {"PosixTzString"}
// Dependencies: {}
# [doc = " A struct for holding data encoded by a POSIX time-zone string."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct PosixTzString { # [doc = " The variant info of the STD time-zone variant."] pub std_info : TimeZoneVariantInfo , # [doc = " The variant info of the DST time-zone variant if present."] pub dst_info : Option < DstTransitionInfo > , }
};
}
