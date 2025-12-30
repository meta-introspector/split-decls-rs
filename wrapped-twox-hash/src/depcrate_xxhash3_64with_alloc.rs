// Generated macro for with_alloc (module)
macro_rules! Depcrate_xxhash3_64with_alloc {
() => {
// Module: crate::xxhash3_64
// Provides: {"with_alloc"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] mod with_alloc { use :: alloc :: boxed :: Box ; use super :: * ; impl Hasher { # [doc = " Constructs the hasher using the default seed and secret values."] pub fn new () -> Self { Self { inner : RawHasherCore :: allocate_default () , _private : () , } } # [doc = " Constructs the hasher using the provided seed and a secret"] # [doc = " derived from the seed."] pub fn with_seed (seed : u64) -> Self { Self { inner : RawHasherCore :: allocate_with_seed (seed) , _private : () , } } # [doc = " Constructs the hasher using the provided seed and secret."] pub fn with_seed_and_secret (seed : u64 , secret : impl Into < Box < [u8] > > ,) -> Result < Self , SecretTooShortError < Box < [u8] > > > { Ok (Self { inner : RawHasherCore :: allocate_with_seed_and_secret (seed , secret) ? , _private : () , }) } # [doc = " Returns the secret."] pub fn into_secret (self) -> Box < [u8] > { self . inner . into_secret () } } impl Default for Hasher { fn default () -> Self { Self :: new () } } impl hash :: Hasher for Hasher { # [inline] fn write (& mut self , input : & [u8]) { self . inner . write (input) } # [inline] fn finish (& self) -> u64 { self . inner . finish (Finalize64) } } }
};
}
