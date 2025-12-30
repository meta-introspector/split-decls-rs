// Generated macro for put_metadata (function)
macro_rules! Depcrateput_metadata {
() => {
// Module: crate
// Provides: {"put_metadata"}
// Dependencies: {}
fn put_metadata (buf : & mut Vec < u8 > , meta : & Metadata , prefix : Option < & str >) { if let Some (prefix) = prefix { write ! (buf , "{}" , prefix) . unwrap () ; } put_field_wellformed (buf , "TARGET" , meta . target () . as_bytes ()) ; if let Some (file) = meta . file () { if let Some (prefix) = prefix { write ! (buf , "{}" , prefix) . unwrap () ; } put_field_wellformed (buf , "CODE_FILE" , file . as_bytes ()) ; } if let Some (line) = meta . line () { if let Some (prefix) = prefix { write ! (buf , "{}" , prefix) . unwrap () ; } writeln ! (buf , "CODE_LINE={}" , line) . unwrap () ; } }
};
}
