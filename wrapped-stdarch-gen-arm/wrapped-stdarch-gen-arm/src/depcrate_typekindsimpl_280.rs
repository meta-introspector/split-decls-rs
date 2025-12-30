// Generated macro for impl_280 (impl)
macro_rules! Depcrate_typekindsimpl_280 {
() => {
// Module: crate::typekinds
// Provides: {"impl_280"}
// Dependencies: {}
impl Ord for TypeKind { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { use std :: cmp :: Ordering :: * ; let self_int : usize = self . into () ; let other_int : usize = other . into () ; if self_int == other_int { match (self , other) { (TypeKind :: Base (ty1) , TypeKind :: Base (ty2)) => ty1 . cmp (ty2) , (TypeKind :: Pointer (ty1 , _) , TypeKind :: Pointer (ty2 , _)) => ty1 . cmp (ty2) , (TypeKind :: Vector (vt1) , TypeKind :: Vector (vt2)) => vt1 . cmp (vt2) , (TypeKind :: Custom (s1) , TypeKind :: Custom (s2)) => s1 . cmp (s2) , (TypeKind :: Wildcard (..) , TypeKind :: Wildcard (..)) => Equal , _ => unreachable ! () , } } else { self_int . cmp (& other_int) } } }
};
}
