// Generated macro for with_alloc (module)
macro_rules! Depcrate_xxhash3_128with_alloc {
() => {
// Module: crate::xxhash3_128
// Provides: {"with_alloc"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] mod with_alloc { use :: alloc :: boxed :: Box ; use super :: * ; impl Hasher { # [doc = " Constructs the hasher using the default seed and secret values."] pub fn new () -> Self { Self { inner : RawHasherCore :: allocate_default () , _private : () , } } # [doc = " Constructs the hasher using the provided seed and a secret"] # [doc = " derived from the seed."] pub fn with_seed (seed : u64) -> Self { Self { inner : RawHasherCore :: allocate_with_seed (seed) , _private : () , } } # [doc = " Constructs the hasher using the provided seed and secret."] pub fn with_seed_and_secret (seed : u64 , secret : impl Into < Box < [u8] > > ,) -> Result < Self , SecretTooShortError < Box < [u8] > > > { Ok (Self { inner : RawHasherCore :: allocate_with_seed_and_secret (seed , secret) ? , _private : () , }) } # [doc = " Returns the secret."] pub fn into_secret (self) -> Box < [u8] > { self . inner . into_secret () } # [doc = " Writes some data into this `Hasher`."] # [inline] pub fn write (& mut self , input : & [u8]) { self . inner . write (input) ; } # [doc = " Returns the hash value for the values written so"] # [doc = " far. Unlike [`std::hash::Hasher::finish`][], this method"] # [doc = " returns the complete 128-bit value calculated, not a"] # [doc = " 64-bit value."] # [inline] pub fn finish_128 (& self) -> u128 { self . inner . finish (Finalize128) } } impl Default for Hasher { fn default () -> Self { Self :: new () } } }
};
}
