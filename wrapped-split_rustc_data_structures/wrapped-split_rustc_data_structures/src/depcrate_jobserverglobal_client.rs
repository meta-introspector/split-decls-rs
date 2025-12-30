// Generated macro for GLOBAL_CLIENT (static)
macro_rules! Depcrate_jobserverGLOBAL_CLIENT {
() => {
// Module: crate::jobserver
// Provides: {"GLOBAL_CLIENT"}
// Dependencies: {}
static GLOBAL_CLIENT : LazyLock < Result < Client , String > > = LazyLock :: new (| | { let FromEnv { client , var } = unsafe { Client :: from_env_ext (true) } ; let error = match client { Ok (client) => return Ok (client) , Err (e) => e , } ; if matches ! (error . kind () , FromEnvErrorKind :: NoEnvVar | FromEnvErrorKind :: NoJobserver | FromEnvErrorKind :: NegativeFd | FromEnvErrorKind :: Unsupported) { return Ok (default_client ()) ; } let (name , value) = var . unwrap () ; Err (format ! ("failed to connect to jobserver from environment variable `{name}={:?}`: {error}" , value)) }) ;
};
}
