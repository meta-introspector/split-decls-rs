// Generated macro for impl_95 (impl)
macro_rules! Depcrate_recomposeimpl_95 {
() => {
// Module: crate::recompose
// Provides: {"impl_95"}
// Dependencies: {}
impl < I : Iterator < Item = char > > Recompositions < I > { # [doc = " Create a new recomposition iterator for canonical compositions (NFC)"] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.nfc()`](crate::UnicodeNormalization::nfc)"] # [doc = " on the iterator."] # [inline] pub fn new_canonical (iter : I) -> Self { Recompositions { iter : Decompositions :: new_canonical (iter) , state : self :: RecompositionState :: Composing , buffer : TinyVec :: new () , composee : None , last_ccc : None , } } # [doc = " Create a new recomposition iterator for compatability compositions (NFkC)"] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.nfkc()`](crate::UnicodeNormalization::nfkc)"] # [doc = " on the iterator."] # [inline] pub fn new_compatible (iter : I) -> Self { Recompositions { iter : Decompositions :: new_compatible (iter) , state : self :: RecompositionState :: Composing , buffer : TinyVec :: new () , composee : None , last_ccc : None , } } }
};
}
