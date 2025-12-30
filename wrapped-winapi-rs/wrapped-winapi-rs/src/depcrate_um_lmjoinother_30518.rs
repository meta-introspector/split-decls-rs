// Generated macro for other_30518 (other)
macro_rules! Depcrate_um_lmjoinother_30518 {
() => {
// Module: crate::um::lmjoin
// Provides: {"other_30518"}
// Dependencies: {}
extern "system" { pub fn NetEnumerateComputerNames (Server : LPCWSTR , NameType : NET_COMPUTER_NAME_TYPE , Reserved : ULONG , EntryCount : PDWORD , ComputerNames : * mut * mut LPWSTR ,) -> NET_API_STATUS ; }
};
}
