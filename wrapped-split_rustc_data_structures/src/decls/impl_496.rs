macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for ! { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { unreachable ! () } }
    };
}

impl_496!()