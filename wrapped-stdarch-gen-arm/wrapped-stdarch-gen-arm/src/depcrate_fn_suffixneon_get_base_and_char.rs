// Generated macro for neon_get_base_and_char (function)
macro_rules! Depcrate_fn_suffixneon_get_base_and_char {
() => {
// Module: crate::fn_suffix
// Provides: {"neon_get_base_and_char"}
// Dependencies: {}
fn neon_get_base_and_char (ty : & VectorType) -> (u32 , char , bool) { let lanes = ty . lanes () ; match ty . base_type () { BaseType :: Sized (BaseTypeKind :: Float , size) => (* size , 'f' , * size * lanes == 128) , BaseType :: Sized (BaseTypeKind :: Int , size) => (* size , 's' , * size * lanes == 128) , BaseType :: Sized (BaseTypeKind :: UInt , size) => (* size , 'u' , * size * lanes == 128) , BaseType :: Sized (BaseTypeKind :: Poly , size) => (* size , 'p' , * size * lanes == 128) , _ => panic ! ("Unhandled {ty:?}") , } }
};
}
