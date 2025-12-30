// Generated macro for impl_290 (impl)
macro_rules! Depcrate_typekindsimpl_290 {
() => {
// Module: crate::typekinds
// Provides: {"impl_290"}
// Dependencies: {}
impl ToRepr for BaseTypeKind { fn repr (& self , repr : TypeRepr) -> String { match (repr , self) { (TypeRepr :: C , Self :: Float) => "float" , (TypeRepr :: C , Self :: Int) => "int" , (TypeRepr :: C , Self :: UInt) => "uint" , (TypeRepr :: C , Self :: Poly) => "poly" , (TypeRepr :: Rust | TypeRepr :: LLVMMachine | TypeRepr :: ACLENotation , Self :: Float) => "f" , (TypeRepr :: Rust , Self :: Int) | (TypeRepr :: LLVMMachine , Self :: Int | Self :: UInt) => "i" , (TypeRepr :: Rust | TypeRepr :: ACLENotation , Self :: UInt) => "u" , (TypeRepr :: Rust | TypeRepr :: LLVMMachine | TypeRepr :: ACLENotation , Self :: Poly) => "p" , (TypeRepr :: ACLENotation , Self :: Int) => "s" , (TypeRepr :: ACLENotation , Self :: Bool) => "b" , (_ , Self :: Bool) => "bool" , _ => { unreachable ! ("no base type kind available for representation {repr:?}") } } . to_string () } }
};
}
