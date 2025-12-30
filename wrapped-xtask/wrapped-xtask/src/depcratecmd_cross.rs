// Generated macro for cmd_cross (function)
macro_rules! Depcratecmd_cross {
() => {
// Module: crate
// Provides: {"cmd_cross"}
// Dependencies: {}
fn cmd_cross () -> Result < () , DynError > { for target in ["i686-unknown-linux-gnu" , "powerpc64-unknown-linux-gnu" ,] { cmd ("cross" , & ["test" , "--workspace" , "--features" , "all" , "--target" , target ,] ,) ? ; } Ok (()) }
};
}
