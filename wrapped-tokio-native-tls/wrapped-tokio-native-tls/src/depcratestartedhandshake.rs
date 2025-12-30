// Generated macro for StartedHandshake (enum)
macro_rules! DepcrateStartedHandshake {
() => {
// Module: crate
// Provides: {"StartedHandshake"}
// Dependencies: {}
enum StartedHandshake < S > { Done (TlsStream < S >) , Mid (MidHandshakeTlsStream < AllowStd < S > >) , }
};
}
