macro_rules! deps {
    () => {
        DecompositionType!();
        Decompositions!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > Decompositions < I > { # [doc = " Create a new decomposition iterator for canonical decompositions (NFD)"] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.nfd()`](crate::UnicodeNormalization::nfd)"] # [doc = " on the iterator."] # [inline] pub fn new_canonical (iter : I) -> Decompositions < I > { Decompositions { kind : self :: DecompositionType :: Canonical , iter : iter . fuse () , buffer : TinyVec :: new () , ready : 0 .. 0 , } } # [doc = " Create a new decomposition iterator for compatability decompositions (NFkD)"] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.nfkd()`](crate::UnicodeNormalization::nfkd)"] # [doc = " on the iterator."] # [inline] pub fn new_compatible (iter : I) -> Decompositions < I > { Decompositions { kind : self :: DecompositionType :: Compatible , iter : iter . fuse () , buffer : TinyVec :: new () , ready : 0 .. 0 , } } }
    };
}

impl_5!();