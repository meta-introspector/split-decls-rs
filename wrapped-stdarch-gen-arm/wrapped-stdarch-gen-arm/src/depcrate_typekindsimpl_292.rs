// Generated macro for impl_292 (impl)
macro_rules! Depcrate_typekindsimpl_292 {
() => {
// Module: crate::typekinds
// Provides: {"impl_292"}
// Dependencies: {}
impl BaseType { pub fn get_size (& self) -> Result < u32 , String > { match self { Self :: Sized (_ , size) => Ok (* size) , _ => Err (format ! ("unexpected invalid base type given {self:#?}")) , } } pub fn kind (& self) -> & BaseTypeKind { match self { BaseType :: Sized (kind , _) | BaseType :: Unsized (kind) => kind , } } pub fn is_bool (& self) -> bool { self . kind () == & BaseTypeKind :: Bool } pub fn is_float (& self) -> bool { self . kind () == & BaseTypeKind :: Float } }
};
}
