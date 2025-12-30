// Generated macro for write (function)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
fn write < S > (dest : & mut String , span : SpanRef < '_ , S > , config : & Config) -> fmt :: Result where S : Subscriber + for < 'span > LookupSpan < 'span > , { if config . module_path { if let Some (module_path) = span . metadata () . module_path () { write ! (dest , "{}::" , module_path) ? ; } } write ! (dest , "{}" , span . name ()) ? ; if config . file_and_line { if let Some (file) = span . metadata () . file () { write ! (dest , ":{}" , file) ? ; } if let Some (line) = span . metadata () . line () { write ! (dest , ":{}" , line) ? ; } } Ok (()) }
};
}
