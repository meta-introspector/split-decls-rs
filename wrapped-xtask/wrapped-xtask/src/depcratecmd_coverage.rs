// Generated macro for cmd_coverage (function)
macro_rules! Depcratecmd_coverage {
() => {
// Module: crate
// Provides: {"cmd_coverage"}
// Dependencies: {}
fn cmd_coverage () -> Result < () , DynError > { cargo (& ["tarpaulin" , "--features" , "all" , "--ignore-tests" , "--out" , "xml" ,]) ? ; cmd ("pycobertura" , & ["show" , "--format" , "html" , "cobertura.xml" , "--output" , "cobertura.html" ,] ,) ? ; cmd ("open" , & ["cobertura.html"]) ? ; Ok (()) }
};
}
