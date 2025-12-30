// Generated macro for ShutdownOrInactive (struct)
macro_rules! Depcrate_serverShutdownOrInactive {
() => {
// Module: crate::server
// Provides: {"ShutdownOrInactive"}
// Dependencies: {}
struct ShutdownOrInactive { rx : mpsc :: Receiver < ServerMessage > , timeout : Option < Pin < Box < Sleep > > > , timeout_dur : Duration , }
};
}
