// Generated macro for impl_133 (impl)
macro_rules! Depcrate_hygieneimpl_133 {
() => {
// Module: crate::hygiene
// Provides: {"impl_133"}
// Dependencies: {}
impl < CTX : HashStableContext > HashStable < CTX > for SyntaxContext { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { const TAG_EXPANSION : u8 = 0 ; const TAG_NO_EXPANSION : u8 = 1 ; if self . is_root () { TAG_NO_EXPANSION . hash_stable (ctx , hasher) ; } else { TAG_EXPANSION . hash_stable (ctx , hasher) ; let (expn_id , transparency) = self . outer_mark () ; expn_id . hash_stable (ctx , hasher) ; transparency . hash_stable (ctx , hasher) ; } } }
};
}
