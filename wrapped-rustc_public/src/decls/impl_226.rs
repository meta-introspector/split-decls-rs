macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_hir :: CoroutineKind { type T = crate :: mir :: CoroutineKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_hir :: { CoroutineDesugaring , CoroutineKind } ; match * self { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: Async , source . stable (tables , cx) ,) } CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: Gen , source . stable (tables , cx) ,) } CoroutineKind :: Coroutine (movability) => { crate :: mir :: CoroutineKind :: Coroutine (movability . stable (tables , cx)) } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: AsyncGen , source . stable (tables , cx) ,) } } } }
    };
}

impl_226!()