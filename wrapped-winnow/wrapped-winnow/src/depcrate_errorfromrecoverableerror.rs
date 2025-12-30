// Generated macro for FromRecoverableError (trait)
macro_rules! Depcrate_errorFromRecoverableError {
() => {
// Module: crate::error
// Provides: {"FromRecoverableError"}
// Dependencies: {}
# [doc = " Capture context from when an error was recovered"] # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub trait FromRecoverableError < I : Stream , E > { # [doc = " Capture context from when an error was recovered"] fn from_recoverable_error (token_start : & < I as Stream > :: Checkpoint , err_start : & < I as Stream > :: Checkpoint , input : & I , e : E ,) -> Self ; }
};
}
