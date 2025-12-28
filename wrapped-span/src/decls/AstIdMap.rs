macro_rules! deps {
    () => {
        ErasedFileAstId!();
        ArenaId!();
    };
}

macro_rules! AstIdMap {
    () => {
        deps!();
        # [doc = " Maps items' `SyntaxNode`s to `ErasedFileAstId`s and back."] # [derive (Default)] pub struct AstIdMap { # [doc = " An arena of the ptrs and their associated ID."] arena : Arena < (SyntaxNodePtr , ErasedFileAstId) > , # [doc = " Map ptr to id."] ptr_map : hashbrown :: HashTable < ArenaId > , # [doc = " Map id to ptr."] id_map : hashbrown :: HashTable < ArenaId > , }
    };
}

AstIdMap!()