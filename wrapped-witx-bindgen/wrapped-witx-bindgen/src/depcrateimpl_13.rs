// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl Render for IntRepr { fn render (& self , src : & mut String) { match self { IntRepr :: U8 => src . push_str ("u8") , IntRepr :: U16 => src . push_str ("u16") , IntRepr :: U32 => src . push_str ("u32") , IntRepr :: U64 => src . push_str ("u64") , } } }
};
}
