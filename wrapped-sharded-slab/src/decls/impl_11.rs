macro_rules! deps {
    () => {
        Config!();
        Pool!();
        Array!();
        Clear!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T > Pool < T > where T : Clear + Default , { # [doc = " Returns a new `Pool` with the default configuration parameters."] pub fn new () -> Self { Self :: new_with_config () } # [doc = " Returns a new `Pool` with the provided configuration parameters."] pub fn new_with_config < C : cfg :: Config > () -> Pool < T , C > { C :: validate () ; Pool { shards : shard :: Array :: new () , _cfg : PhantomData , } } }
    };
}

impl_11!();