// Generated macro for FnSig (struct)
macro_rules! Depcrate_itemsFnSig {
() => {
// Module: crate::items
// Provides: {"FnSig"}
// Dependencies: {}
# [doc = " Represents a fn's signature."] pub (crate) struct FnSig < 'a > { decl : & 'a ast :: FnDecl , generics : & 'a ast :: Generics , ext : ast :: Extern , coroutine_kind : Cow < 'a , Option < ast :: CoroutineKind > > , constness : ast :: Const , defaultness : ast :: Defaultness , safety : ast :: Safety , visibility : & 'a ast :: Visibility , }
};
}
