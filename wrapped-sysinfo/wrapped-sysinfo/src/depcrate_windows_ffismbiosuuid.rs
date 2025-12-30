// Generated macro for SMBIOSUuid (struct)
macro_rules! Depcrate_windows_ffiSMBIOSUuid {
() => {
// Module: crate::windows::ffi
// Provides: {"SMBIOSUuid"}
// Dependencies: {}
# [repr (C , packed)] pub struct SMBIOSUuid { pub time_low : u32 , pub time_mid : u16 , pub time_hi_and_version : u16 , pub clock_seq_hi_and_reserved : u8 , pub clock_seq_low : u8 , pub node : [u8 ; 6] , }
};
}
