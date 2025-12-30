// Generated macro for SMBIOSType (trait)
macro_rules! Depcrate_windows_ffiSMBIOSType {
() => {
// Module: crate::windows::ffi
// Provides: {"SMBIOSType"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Only implement for SMBIOS struct types where any initialized byte sequence"] # [doc = " is a valid instance. No extra invariants beyond byte layout."] pub unsafe trait SMBIOSType { fn length (& self) -> u8 ; }
};
}
