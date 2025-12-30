// Generated macro for impl_210 (impl)
macro_rules! Depcrate_matchingimpl_210 {
() => {
// Module: crate::matching
// Provides: {"impl_210"}
// Dependencies: {}
impl < T > MatchKindValues < T > { pub fn get (& mut self , ty : & TypeKind , ctx : & LocalContext) -> context :: Result < & T > { let base_ty = if let Some (w) = ty . wildcard () { ctx . provide_type_wildcard (w) ? } else { ty . clone () } ; match (base_ty . base_type () . unwrap () . kind () , & self . float , & self . unsigned ,) { (BaseTypeKind :: Float , Some (v) , _) | (BaseTypeKind :: UInt , _ , Some (v)) => Ok (v) , _ => Ok (& self . default) , } } }
};
}
