macro_rules! seed {
    () => {
        pub fn seed () -> u64 { use std :: hash :: { BuildHasher , Hasher } ; # [allow (clippy :: disallowed_types)] std :: collections :: hash_map :: RandomState :: new () . build_hasher () . finish () }
    };
}

seed!()