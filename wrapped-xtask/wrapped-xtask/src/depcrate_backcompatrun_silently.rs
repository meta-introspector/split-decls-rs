// Generated macro for run_silently (function)
macro_rules! Depcrate_backcompatrun_silently {
() => {
// Module: crate::backcompat
// Provides: {"run_silently"}
// Dependencies: {}
fn run_silently (command : & mut Command , err : impl FnOnce () -> anyhow :: Error) -> anyhow :: Result < () > { let output = command . output () ? ; if ! output . status . success () { let formatted_command = format ! ("{command:?}") ; if ! output . stdout . is_empty () { println ! ("stdout:\n{}" , std :: str :: from_utf8 (& output . stdout) . map_err (| e | anyhow ! ("`{}` output is not UTF-8: {}" , formatted_command , e)) ?) ; } if ! output . stderr . is_empty () { println ! ("stderr:\n{}" , std :: str :: from_utf8 (& output . stderr) . map_err (| e | anyhow ! ("`{}` output is not UTF-8: {}" , formatted_command , e)) ?) ; } println ! ("exit-code: {}" , output . status . code () . map (| code | code . to_string () . into ()) . unwrap_or (Cow :: Borrowed ("non-zero"))) ; return Err (err ()) ; } Ok (()) }
};
}
