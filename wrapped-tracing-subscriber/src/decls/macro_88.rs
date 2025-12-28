macro_rules! deps {
    () => {
        LookupSpan!();
        Scope!();
        SpanRef!();
    };
}

macro_rules! macro_88 {
    () => {
        deps!();
        feature ! { #! [any (feature = "alloc" , feature = "std")] # [cfg (not (feature = "smallvec"))] use alloc :: vec :: { self , Vec } ; use core :: { fmt , iter } ; # [doc = " An iterator over the parents of a span, ordered from root to leaf."] # [doc = ""] # [doc = " This is returned by the [`Scope::from_root`] method."] pub struct ScopeFromRoot <'a , R > where R : LookupSpan <'a >, { # [cfg (feature = "smallvec")] spans : iter :: Rev < smallvec :: IntoIter < SpanRef <'a , R >, 16 >>, # [cfg (not (feature = "smallvec"))] spans : iter :: Rev < vec :: IntoIter < SpanRef <'a , R >>>, } # [cfg (feature = "smallvec")] type SpanRefVecArray <'span , L > = [SpanRef <'span , L >; 16] ; impl <'a , R > Scope <'a , R > where R : LookupSpan <'a >, { # [doc = " Flips the order of the iterator, so that it is ordered from root to leaf."] # [doc = ""] # [doc = " The iterator will first return the root span, then that span's immediate child,"] # [doc = " and so on until it finally returns the span that [`SpanRef::scope`] was called on."] # [doc = ""] # [doc = " If any items were consumed from the [`Scope`] before calling this method then they"] # [doc = " will *not* be returned from the [`ScopeFromRoot`]."] # [doc = ""] # [doc = " **Note**: this will allocate if there are many spans remaining, or if the"] # [doc = " \"smallvec\" feature flag is not enabled."] # [allow (clippy :: wrong_self_convention)] pub fn from_root (self) -> ScopeFromRoot <'a , R > { # [cfg (feature = "smallvec")] type Buf < T > = smallvec :: SmallVec < T , 16 >; # [cfg (not (feature = "smallvec"))] type Buf < T > = Vec < T >; ScopeFromRoot { spans : self . collect ::< Buf < _ >> () . into_iter () . rev () , } } } impl <'a , R > Iterator for ScopeFromRoot <'a , R > where R : LookupSpan <'a >, { type Item = SpanRef <'a , R >; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . spans . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . spans . size_hint () } } impl <'a , R > fmt :: Debug for ScopeFromRoot <'a , R > where R : LookupSpan <'a >, { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . pad ("ScopeFromRoot { .. }") } } }
    };
}

macro_88!();