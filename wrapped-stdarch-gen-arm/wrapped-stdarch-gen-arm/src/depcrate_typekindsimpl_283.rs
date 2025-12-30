// Generated macro for impl_283 (impl)
macro_rules! Depcrate_typekindsimpl_283 {
() => {
// Module: crate::typekinds
// Provides: {"impl_283"}
// Dependencies: {}
impl ToRepr for VectorType { fn repr (& self , repr : TypeRepr) -> String { let make_llvm_repr = | show_unsigned | { format ! ("{}v{}{}" , if self . is_scalable { "nx" } else { "" } , self . lanes * (self . tuple_size . map (usize :: from) . unwrap_or (1) as u32) , match self . base_type { BaseType :: Sized (BaseTypeKind :: UInt , size) if show_unsigned => format ! ("u{size}") , _ => self . base_type . llvm_machine_repr () , }) } ; if matches ! (repr , TypeRepr :: ACLENotation) { self . base_type . acle_notation_repr () } else if matches ! (repr , TypeRepr :: LLVMMachine) { make_llvm_repr (false) } else if self . is_scalable { match (self . base_type , self . lanes , self . tuple_size) { (BaseType :: Sized (BaseTypeKind :: Bool , _) , 16 , _) => "svbool_t" . to_string () , (BaseType :: Sized (BaseTypeKind :: Bool , _) , lanes , _) => format ! ("svbool{lanes}_t") , (BaseType :: Sized (_ , size) , lanes , _) if VECTOR_FULL_REGISTER_SIZE != (size * lanes) => { make_llvm_repr (true) } (ty , _ , None) => format ! ("sv{}_t" , ty . c_repr ()) , (ty , _ , Some (tuple_size)) => format ! ("sv{}x{tuple_size}_t" , ty . c_repr ()) , } } else { match self . tuple_size { Some (tuple_size) => format ! ("{}x{}x{}_t" , self . base_type . c_repr () , self . lanes , tuple_size) , None => format ! ("{}x{}_t" , self . base_type . c_repr () , self . lanes) , } } } }
};
}
