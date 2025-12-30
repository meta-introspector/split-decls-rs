// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let result = SPANISH_LIST_FORMATTER . get_or_init (| | { ListFormatter :: try_new_and (locale ! ("es") . into () , Default :: default ()) . expect ("locale 'es' should be present in compiled data") }) . format_to_string (["uno" , "dos" , "tres"] . iter ()) ; assert_eq ! (result , "uno, dos y tres") ; println ! ("{result}") ; }
};
}
