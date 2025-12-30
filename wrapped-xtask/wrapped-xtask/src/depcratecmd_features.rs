// Generated macro for cmd_features (function)
macro_rules! Depcratecmd_features {
() => {
// Module: crate
// Provides: {"cmd_features"}
// Dependencies: {}
fn cmd_features () -> Result < () , DynError > { cargo (& ["test" , "--workspace"]) ? ; cargo (& ["test" , "-p" , "object" , "--no-default-features"]) ? ; cargo (& ["test" , "-p" , "object-examples" , "--no-default-features"]) ? ; cargo (& ["test" , "-p" , "object-rewrite" , "--no-default-features"]) ? ; for features in ["read" , "write" , "build" , "read_core,write_core,coff" , "read_core,write_core,build_core,elf" , "read_core,write_core,macho" , "read_core,write_core,pe" , "read_core,write_core,xcoff" , "read_core,wasm" , "std" , "compression" , "unaligned" ,] { cargo (& ["test" , "-p" , "object" , "-p" , "object-examples" , "--no-default-features" , "--features" , features ,]) ? ; } cargo (& ["test" , "-p" , "object-rewrite" , "--no-default-features" , "--features" , "logging" ,]) ? ; Ok (()) }
};
}
