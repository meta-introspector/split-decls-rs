macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a > HashStable < StableHashingContext < 'a > > for ast :: NodeId { # [inline] fn hash_stable (& self , _ : & mut StableHashingContext < 'a > , _ : & mut StableHasher) { panic ! ("Node IDs should not appear in incremental state") ; } }
    };
}

impl_119!()