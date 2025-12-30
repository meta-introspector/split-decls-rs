// Generated macro for SMBIOSSystemInformation (struct)
macro_rules! Depcrate_windows_ffiSMBIOSSystemInformation {
() => {
// Module: crate::windows::ffi
// Provides: {"SMBIOSSystemInformation"}
// Dependencies: {}
# [repr (C , packed)] pub (crate) struct SMBIOSSystemInformation { pub (crate) _type : u8 , pub (crate) length : u8 , pub (crate) _handle : u16 , pub (crate) manufacturer : u8 , pub (crate) product_name : u8 , pub (crate) version : u8 , pub (crate) serial_number : u8 , pub (crate) uuid : SMBIOSUuid , pub (crate) wake_up_type : u8 , pub (crate) sku_number : u8 , pub (crate) family : u8 , }
};
}
