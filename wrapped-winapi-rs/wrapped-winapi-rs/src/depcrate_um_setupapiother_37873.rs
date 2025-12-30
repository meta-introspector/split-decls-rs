// Generated macro for other_37873 (other)
macro_rules! Depcrate_um_setupapiother_37873 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37873"}
// Dependencies: {}
extern "system" { pub fn SetupOpenLog (Erase : BOOL ,) -> BOOL ; pub fn SetupLogErrorA (MessageString : LPCSTR , Severity : LogSeverity ,) -> BOOL ; pub fn SetupLogErrorW (MessageString : LPCWSTR , Severity : LogSeverity ,) -> BOOL ; pub fn SetupCloseLog () -> () ; pub fn SetupGetThreadLogToken () -> SP_LOG_TOKEN ; pub fn SetupSetThreadLogToken (LogToken : SP_LOG_TOKEN ,) -> () ; }
};
}
