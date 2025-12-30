// Generated macro for impl_63 (impl)
macro_rules! Depcrate_deimpl_63 {
() => {
// Module: crate::de
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'de , 'a , T : ? Sized + 'static > Visitor < 'de > for MapLookupVisitor < 'a , T > { type Value = DeserializeFn < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Expected :: fmt (self . expected , formatter) } fn visit_str < E > (self , key : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match self . registry . map . get (key) { Some (Some (value)) => Ok (* value) , Some (None) => Err (de :: Error :: custom (format_args ! ("non-unique tag of {}: {:?}" , self . expected , key))) , None => Err (de :: Error :: unknown_variant (key , & self . registry . names)) , } } }
};
}
