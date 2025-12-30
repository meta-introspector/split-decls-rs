// Generated macro for make_imports (function)
macro_rules! Depcratemake_imports {
() => {
// Module: crate
// Provides: {"make_imports"}
// Dependencies: {}
pub fn make_imports () -> Result < Object , JsValue > { let map = Map :: new () ; let imports : JsValue = Imports . into () ; bind (& imports , "native_add") ? ; map . set (& JsValue :: from ("env") , & imports) ; Object :: from_entries (& map . into ()) }
};
}
