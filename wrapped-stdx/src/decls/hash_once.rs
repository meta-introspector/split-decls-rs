macro_rules! hash_once {
    () => {
        pub fn hash_once < Hasher : std :: hash :: Hasher + Default > (thing : impl std :: hash :: Hash) -> u64 { std :: hash :: BuildHasher :: hash_one (& std :: hash :: BuildHasherDefault :: < Hasher > :: default () , thing) }
    };
}

hash_once!();