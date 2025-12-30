// Generated macro for other_30036 (other)
macro_rules! Depcrate_um_lmalertother_30036 {
() => {
// Module: crate::um::lmalert
// Provides: {"other_30036"}
// Dependencies: {}
extern "system" { pub fn NetAlertRaise (AlertType : LPCWSTR , Buffer : LPVOID , BufferSize : DWORD ,) -> NET_API_STATUS ; pub fn NetAlertRaiseEx (AlertType : LPCWSTR , VariableInfo : LPVOID , VariableInfoSize : DWORD , ServiceName : LPCWSTR ,) -> NET_API_STATUS ; }
};
}
