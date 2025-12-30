// Generated macro for list_deref (function)
macro_rules! Depcratelist_deref {
() => {
// Module: crate
// Provides: {"list_deref"}
// Dependencies: {}
# [doc = " List extended attributes attached to the specified file (dereference symlinks)."] pub fn list_deref < P > (path : P) -> io :: Result < XAttrs > where P : AsRef < Path > , { sys :: list_path (path . as_ref () , true) }
};
}
