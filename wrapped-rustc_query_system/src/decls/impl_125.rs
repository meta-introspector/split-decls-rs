macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a > HashStable < StableHashingContext < 'a > > for [hir :: Attribute] { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { if self . is_empty () { self . len () . hash_stable (hcx , hasher) ; return ; } let filtered : SmallVec < [& hir :: Attribute ; 8] > = self . iter () . filter (| attr | { ! attr . is_doc_comment () && ! attr . ident () . is_some_and (| ident | hcx . is_ignored_attr (ident . name)) }) . collect () ; filtered . len () . hash_stable (hcx , hasher) ; for attr in filtered { attr . hash_stable (hcx , hasher) ; } } }
    };
}

impl_125!();