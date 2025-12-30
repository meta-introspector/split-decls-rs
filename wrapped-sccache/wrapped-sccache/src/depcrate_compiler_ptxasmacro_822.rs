// Generated macro for macro_822 (macro)
macro_rules! Depcrate_compiler_ptxasmacro_822 {
() => {
// Module: crate::compiler::ptxas
// Provides: {"macro_822"}
// Dependencies: {}
counted_array ! (pub static ARGS : [ArgInfo < cicc :: ArgData >; _] = [take_arg ! ("-arch" , OsString , CanBeSeparated (b'=') , PassThrough) , take_arg ! ("-m" , OsString , CanBeSeparated (b'=') , PassThrough) , take_arg ! ("-o" , PathBuf , Separated , Output) ,]) ;
};
}
