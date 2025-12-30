// Generated macro for is_subpath (function)
macro_rules! Depcrate_testis_subpath {
() => {
// Module: crate::test
// Provides: {"is_subpath"}
// Dependencies: {}
fn is_subpath < P > (path : & Path , subpath : & P) -> bool where P : AsRef < Path > , { (0 .. path . components () . count ()) . map (| i | { path . components () . skip (i) . take (subpath . as_ref () . components () . count ()) }) . any (| c | c . zip (subpath . as_ref () . components ()) . all (| (a , b) | a == b)) }
};
}
