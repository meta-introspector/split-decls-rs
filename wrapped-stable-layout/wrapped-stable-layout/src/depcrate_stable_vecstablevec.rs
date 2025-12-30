// Generated macro for StableVec (struct)
macro_rules! Depcrate_stable_vecStableVec {
() => {
// Module: crate::stable_vec
// Provides: {"StableVec"}
// Dependencies: {}
# [doc = " `Vec`, with a stable memory layout"] # [doc = ""] # [doc = " This container is used within the runtime to ensure memory mapping and memory accesses are"] # [doc = " valid.  We rely on known addresses and offsets within the runtime, and since `Vec`'s layout"] # [doc = " is allowed to change, we must provide a way to lock down the memory layout.  `StableVec`"] # [doc = " reimplements the bare minimum of `Vec`'s API sufficient only for the runtime's needs."] # [doc = ""] # [doc = " To ensure memory allocation and deallocation is handled correctly, it is only possible to"] # [doc = " create a new `StableVec` from an existing `Vec`.  This way we ensure all Rust invariants are"] # [doc = " upheld."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creating a `StableVec` from a `Vec`"] # [doc = ""] # [doc = " ```"] # [doc = " # use solana_stable_layout::stable_vec::StableVec;"] # [doc = " let vec = vec![\"meow\", \"woof\", \"moo\"];"] # [doc = " let vec = StableVec::from(vec);"] # [doc = " ```"] # [repr (C)] pub struct StableVec < T > { pub addr : u64 , pub cap : u64 , pub len : u64 , _marker : PhantomData < T > , }
};
}
