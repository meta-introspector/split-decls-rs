macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T > Slab < T > { # [doc = " Returns a new slab with the default configuration parameters."] pub fn new () -> Self { Self :: new_with_config () } # [doc = " Returns a new slab with the provided configuration parameters."] pub fn new_with_config < C : cfg :: Config > () -> Slab < T , C > { C :: validate () ; Slab { shards : shard :: Array :: new () , _cfg : PhantomData , } } }
    };
}

impl_14!()