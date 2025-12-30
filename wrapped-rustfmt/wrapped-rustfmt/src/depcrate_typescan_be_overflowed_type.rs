// Generated macro for can_be_overflowed_type (function)
macro_rules! Depcrate_typescan_be_overflowed_type {
() => {
// Module: crate::types
// Provides: {"can_be_overflowed_type"}
// Dependencies: {}
pub (crate) fn can_be_overflowed_type (context : & RewriteContext < '_ > , ty : & ast :: Ty , len : usize ,) -> bool { match ty . kind { ast :: TyKind :: Tup (..) => context . use_block_indent () && len == 1 , ast :: TyKind :: Ref (_ , ref mutty) | ast :: TyKind :: PinnedRef (_ , ref mutty) | ast :: TyKind :: Ptr (ref mutty) => can_be_overflowed_type (context , & * mutty . ty , len) , _ => false , } }
};
}
