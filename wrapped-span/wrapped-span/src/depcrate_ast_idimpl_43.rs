// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ast_idimpl_43 {
() => {
// Module: crate::ast_id
// Provides: {"impl_43"}
// Dependencies: {}
impl ErasedAstIdNextIndexMap { # [inline] fn new_id (& mut self , kind : ErasedFileAstIdKind , data : impl Hash) -> ErasedFileAstId { let hash = FxBuildHasher . hash_one (& data) ; let initial_hash = u16_hash (hash) ; let mut hash = initial_hash ; let index = loop { match self . 0 . entry ((kind , hash)) { std :: collections :: hash_map :: Entry :: Occupied (mut entry) => { let i = entry . get_mut () ; if * i < ((1 << INDEX_BITS) - 1) { * i += 1 ; break * i ; } } std :: collections :: hash_map :: Entry :: Vacant (entry) => { entry . insert (0) ; break 0 ; } } hash = hash . wrapping_add (1) ; if hash == initial_hash { panic ! ("you have way too many items in the same file!") ; } } ; let kind = kind as u32 ; ErasedFileAstId (pack_hash_index_and_kind (hash , index , kind)) } }
};
}
