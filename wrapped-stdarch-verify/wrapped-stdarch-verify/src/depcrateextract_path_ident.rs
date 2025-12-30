// Generated macro for extract_path_ident (function)
macro_rules! Depcrateextract_path_ident {
() => {
// Module: crate
// Provides: {"extract_path_ident"}
// Dependencies: {}
fn extract_path_ident (path : & syn :: Path) -> syn :: Ident { if path . leading_colon . is_some () { panic ! ("unsupported leading colon in path") } if path . segments . len () != 1 { panic ! ("unsupported path that needs name resolution") } match path . segments . first () . expect ("segment not found") . arguments { syn :: PathArguments :: None => { } _ => panic ! ("unsupported path that has path arguments") , } path . segments . first () . expect ("segment not found") . ident . clone () }
};
}
