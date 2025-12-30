// Generated macro for cmd_msrv (function)
macro_rules! Depcratecmd_msrv {
() => {
// Module: crate
// Provides: {"cmd_msrv"}
// Dependencies: {}
fn cmd_msrv () -> Result < () , DynError > { cmd_with ("cargo" , & ["+1.65.0" , "test" , "-p" , "object" , "--no-default-features" , "--features" , "read,write,build,std"] , | cmd | { cmd . env ("CARGO_NET_GIT_FETCH_WITH_CLI" , "true") ; } ,) ? ; cmd_with ("cargo" , & ["+1.81.0" , "test" , "-p" , "object" , "--features" , "all"] , | cmd | { cmd . env ("CARGO_NET_GIT_FETCH_WITH_CLI" , "true") ; } ,) ? ; Ok (()) }
};
}
