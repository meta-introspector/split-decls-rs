// Generated macro for impl_54 (impl)
macro_rules! Depcrate_expandimpl_54 {
() => {
// Module: crate::expand
// Provides: {"impl_54"}
// Dependencies: {}
impl RecordType { # [doc = " Array of primitive types which should be recorded as [RecordType::Value]."] const TYPES_FOR_VALUE : & 'static [& 'static str] = & ["bool" , "str" , "u8" , "i8" , "u16" , "i16" , "u32" , "i32" , "u64" , "i64" , "u128" , "i128" , "f32" , "f64" , "usize" , "isize" , "String" , "NonZeroU8" , "NonZeroI8" , "NonZeroU16" , "NonZeroI16" , "NonZeroU32" , "NonZeroI32" , "NonZeroU64" , "NonZeroI64" , "NonZeroU128" , "NonZeroI128" , "NonZeroUsize" , "NonZeroIsize" , "Wrapping" ,] ; # [doc = " Parse `RecordType` from [Type] by looking up"] # [doc = " the [RecordType::TYPES_FOR_VALUE] array."] fn parse_from_ty (ty : & Type) -> Self { match ty { Type :: Path (TypePath { path , .. }) if path . segments . iter () . next_back () . map (| path_segment | { let ident = path_segment . ident . to_string () ; Self :: TYPES_FOR_VALUE . iter () . any (| & t | t == ident) }) . unwrap_or (false) => { RecordType :: Value } Type :: Reference (syn :: TypeReference { elem , .. }) => RecordType :: parse_from_ty (elem) , _ => RecordType :: Debug , } } }
};
}
