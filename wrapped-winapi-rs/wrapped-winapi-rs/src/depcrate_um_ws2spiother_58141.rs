// Generated macro for other_58141 (other)
macro_rules! Depcrate_um_ws2spiother_58141 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58141"}
// Dependencies: {}
extern "system" { # [cfg (target_pointer_width = "64")] pub fn WSCEnumProtocols32 (lpiProtocols : LPINT , lpProtocolBuffer : LPWSAPROTOCOL_INFOW , lpdwBufferLength : LPDWORD , lpErrno : LPINT ,) -> c_int ; pub fn WSCDeinstallProvider (lpProviderId : LPGUID , lpErrno : LPINT ,) -> c_int ; }
};
}
