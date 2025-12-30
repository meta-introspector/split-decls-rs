// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Render for BuiltinType { fn render (& self , src : & mut String) { match self { BuiltinType :: U8 { lang_c_char : _ } => src . push_str ("u8") , BuiltinType :: U16 => src . push_str ("u16") , BuiltinType :: U32 { lang_ptr_size : false , } => src . push_str ("u32") , BuiltinType :: U32 { lang_ptr_size : true , } => src . push_str ("usize") , BuiltinType :: U64 => src . push_str ("u64") , BuiltinType :: S8 => src . push_str ("i8") , BuiltinType :: S16 => src . push_str ("i16") , BuiltinType :: S32 => src . push_str ("i32") , BuiltinType :: S64 => src . push_str ("i64") , BuiltinType :: F32 => src . push_str ("f32") , BuiltinType :: F64 => src . push_str ("f64") , BuiltinType :: Char => src . push_str ("char") , } } }
};
}
