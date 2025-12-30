// Generated macro for pack_hash_index_and_kind (function)
macro_rules! Depcrate_ast_idpack_hash_index_and_kind {
() => {
// Module: crate::ast_id
// Provides: {"pack_hash_index_and_kind"}
// Dependencies: {}
# [inline] const fn pack_hash_index_and_kind (hash : u16 , index : u32 , kind : u32) -> u32 { (hash as u32) | (index << HASH_BITS) | (kind << (HASH_BITS + INDEX_BITS)) }
};
}
