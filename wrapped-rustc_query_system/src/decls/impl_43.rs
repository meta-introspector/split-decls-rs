macro_rules! impl_43 {
    () => {
        impl DepNodeIndex { const SINGLETON_ZERO_DEPS_ANON_NODE : DepNodeIndex = DepNodeIndex :: ZERO ; pub const FOREVER_RED_NODE : DepNodeIndex = DepNodeIndex :: from_u32 (1) ; }
    };
}

impl_43!()