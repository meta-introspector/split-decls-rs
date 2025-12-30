// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl Render for WasmType { fn render (& self , src : & mut String) { match self { WasmType :: I32 => src . push_str ("i32") , WasmType :: I64 => src . push_str ("i64") , WasmType :: F32 => src . push_str ("f32") , WasmType :: F64 => src . push_str ("f64") , } } }
};
}
