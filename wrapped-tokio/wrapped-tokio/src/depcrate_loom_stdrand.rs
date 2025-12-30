// Generated macro for rand (module)
macro_rules! Depcrate_loom_stdrand {
() => {
// Module: crate::loom::std
// Provides: {"rand"}
// Dependencies: {}
pub (crate) mod rand { use std :: collections :: hash_map :: RandomState ; use std :: hash :: { BuildHasher , Hash , Hasher } ; use std :: sync :: atomic :: AtomicU32 ; use std :: sync :: atomic :: Ordering :: Relaxed ; static COUNTER : AtomicU32 = AtomicU32 :: new (1) ; pub (crate) fn seed () -> u64 { let rand_state = RandomState :: new () ; rand_state . hash_one (COUNTER . fetch_add (1 , Relaxed)) } }
};
}
