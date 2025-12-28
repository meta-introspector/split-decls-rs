macro_rules! deps {
    () => {
        ErasedFileAstId!();
        ErasedAstIdNextIndexMap!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ErasedFileAstId { # [inline] fn hash_value (self) -> u16 { self . 0 as u16 } # [inline] fn index (self) -> u32 { (self . 0 << KIND_BITS) >> (HASH_BITS + KIND_BITS) } # [inline] fn kind (self) -> u32 { self . 0 >> (HASH_BITS + INDEX_BITS) } fn ast_id_for (node : & SyntaxNode , index_map : & mut ErasedAstIdNextIndexMap , parent : Option < & ErasedFileAstId > ,) -> Option < ErasedFileAstId > { has_name_ast_id (node , index_map) . or_else (| | assoc_item_ast_id (node , index_map , parent)) . or_else (| | extern_block_ast_id (node , index_map)) . or_else (| | use_ast_id (node , index_map)) . or_else (| | impl_ast_id (node , index_map)) . or_else (| | asm_expr_ast_id (node , index_map)) } fn should_alloc (node : & SyntaxNode) -> bool { let kind = node . kind () ; should_alloc_has_name (kind) || should_alloc_assoc_item (kind) || ast :: ExternBlock :: can_cast (kind) || ast :: Use :: can_cast (kind) || ast :: Impl :: can_cast (kind) || ast :: AsmExpr :: can_cast (kind) } # [inline] pub fn into_raw (self) -> u32 { self . 0 } # [inline] pub const fn from_raw (v : u32) -> Self { Self (v) } }
    };
}

impl_12!();