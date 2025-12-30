// Generated macro for StartedState (enum)
macro_rules! Depcrate_http_compatStartedState {
() => {
// Module: crate::http_compat
// Provides: {"StartedState"}
// Dependencies: {}
enum StartedState < T > { Unstarted (T) , Started { # [allow (dead_code)] result : wit_bindgen :: FutureWriter < Result < () , ErrorCode > > , state : IncomingState , } , Empty , }
};
}
