macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'ctx > rustc_hir :: HashStableContext for StableHashingContext < 'ctx > { fn hash_attr_id (& mut self , _id : & HashIgnoredAttrId , _hasher : & mut StableHasher) { } }
    };
}

impl_126!();