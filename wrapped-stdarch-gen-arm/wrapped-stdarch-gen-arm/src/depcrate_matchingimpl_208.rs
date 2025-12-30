// Generated macro for impl_208 (impl)
macro_rules! Depcrate_matchingimpl_208 {
() => {
// Module: crate::matching
// Provides: {"impl_208"}
// Dependencies: {}
impl < T > MatchSizeValues < T > { pub fn get (& mut self , ty : & TypeKind , ctx : & LocalContext) -> context :: Result < & T > { let base_ty = if let Some (w) = ty . wildcard () { ctx . provide_type_wildcard (w) ? } else { ty . clone () } ; if let BaseType :: Sized (_ , bitsize) = base_ty . base_type () . unwrap () { match (bitsize , & self . byte , & self . halfword , & self . doubleword) { (64 , _ , _ , Some (v)) | (16 , _ , Some (v) , _) | (8 , Some (v) , _ , _) => Ok (v) , _ => Ok (& self . default) , } } else { Err (format ! ("cannot match bitsize to unsized type {ty:?}!")) } } }
};
}
