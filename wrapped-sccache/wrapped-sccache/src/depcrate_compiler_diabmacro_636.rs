// Generated macro for macro_636 (macro)
macro_rules! Depcrate_compiler_diabmacro_636 {
() => {
// Module: crate::compiler::diab
// Provides: {"macro_636"}
// Dependencies: {}
counted_array ! (pub static ARGS : [ArgInfo < ArgData >; _] = [flag ! ("-" , TooHardFlag) , flag ! ("-##" , TooHardFlag) , flag ! ("-###" , TooHardFlag) , take_arg ! ("-@" , OsString , Concatenated , TooHard) , take_arg ! ("-D" , OsString , CanBeSeparated , PreprocessorArgument) , flag ! ("-E" , TooHardFlag) , take_arg ! ("-I" , PathBuf , CanBeSeparated , PreprocessorArgumentPath) , take_arg ! ("-L" , OsString , Separated , PassThrough) , flag ! ("-P" , TooHardFlag) , flag ! ("-S" , TooHardFlag) , take_arg ! ("-U" , OsString , CanBeSeparated , PreprocessorArgument) , flag ! ("-V" , TooHardFlag) , flag ! ("-VV" , TooHardFlag) , take_arg ! ("-W" , OsString , Separated , PassThrough) , flag ! ("-Xmake-dependency" , DepArgumentFlag) , flag ! ("-Xmake-dependency-canonicalize-path-off" , DepArgumentFlag) , take_arg ! ("-Xmake-dependency-savefile" , PathBuf , Concatenated (b'=') , DepArgumentPath) , take_arg ! ("-Xmake-dependency-target" , OsString , Concatenated (b'=') , DepArgument) , flag ! ("-c" , DoCompilation) , take_arg ! ("-include" , PathBuf , CanBeSeparated , PreprocessorArgumentPath) , take_arg ! ("-l" , OsString , Separated , PassThrough) , take_arg ! ("-o" , PathBuf , Separated , Output) , take_arg ! ("-t" , OsString , Separated , PassThrough) ,]) ;
};
}
