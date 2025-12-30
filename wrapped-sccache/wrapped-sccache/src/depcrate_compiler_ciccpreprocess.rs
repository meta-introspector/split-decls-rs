// Generated macro for preprocess (function)
macro_rules! Depcrate_compiler_ciccpreprocess {
() => {
// Module: crate::compiler::cicc
// Provides: {"preprocess"}
// Dependencies: {}
pub async fn preprocess (cwd : & Path , parsed_args : & ParsedArguments) -> Result < process :: Output > { let input = if parsed_args . input . is_absolute () { parsed_args . input . clone () } else { cwd . join (& parsed_args . input) } ; std :: fs :: read (input) . map_err (anyhow :: Error :: new) . map (| s | process :: Output { status : process :: ExitStatus :: default () , stdout : s , stderr : vec ! [] , }) }
};
}
