// Generated macro for impl_679 (impl)
macro_rules! Depcrate_zalsaimpl_679 {
() => {
// Module: crate::zalsa
// Provides: {"impl_679"}
// Dependencies: {}
impl ErasedJar { # [doc = " Performs type-erasure of a given ingredient."] pub const fn erase < I : HasJar > () -> Self { Self { kind : I :: KIND , # [allow (clippy :: incompatible_msrv)] type_id : TypeId :: of :: < I :: Jar > , type_name : std :: any :: type_name :: < I :: Jar > , create_ingredients : < I :: Jar > :: create_ingredients , id_struct_type_id : < I :: Jar > :: id_struct_type_id , } } pub fn type_name (& self) -> & 'static str { (self . type_name) () } }
};
}
