// Generated macro for impl_294 (impl)
macro_rules! Depcrate_typekindsimpl_294 {
() => {
// Module: crate::typekinds
// Provides: {"impl_294"}
// Dependencies: {}
impl ToRepr for BaseType { fn repr (& self , repr : TypeRepr) -> String { use BaseType :: * ; use BaseTypeKind :: * ; use TypeRepr :: * ; match (self , & repr) { (Sized (Bool , _) | Unsized (Bool) , LLVMMachine) => "i1" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 8 => "b" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 16 => "h" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 32 => "w" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 64 => "d" . to_string () , (Sized (_ , size) , SizeLiteral) if * size == 128 => "q" . to_string () , (_ , SizeLiteral) => unreachable ! ("cannot represent {self:#?} as size literal") , (Sized (Float , _) | Unsized (Float) , TypeKind) => "f" . to_string () , (Sized (Int , _) | Unsized (Int) , TypeKind) => "s" . to_string () , (Sized (UInt , _) | Unsized (UInt) , TypeKind) => "u" . to_string () , (Sized (_ , size) , Size) => size . to_string () , (Sized (_ , size) , SizeInBytesLog2) => { assert ! (size . is_power_of_two () && * size >= 8) ; (size >> 3) . trailing_zeros () . to_string () } (Sized (kind , size) , _) => format ! ("{}{size}" , kind . repr (repr)) , (Unsized (kind) , _) => kind . repr (repr) , } } }
};
}
