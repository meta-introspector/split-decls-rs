// Generated macro for StableCompare (trait)
macro_rules! Depcrate_stable_hasherStableCompare {
() => {
// Module: crate::stable_hasher
// Provides: {"StableCompare"}
// Dependencies: {}
# [doc = " This is a companion trait to `StableOrd`. Some types like `Symbol` can be"] # [doc = " compared in a cross-session stable way, but their `Ord` implementation is"] # [doc = " not stable. In such cases, a `StableOrd` implementation can be provided"] # [doc = " to offer a lightweight way for stable sorting. (The more heavyweight option"] # [doc = " is to sort via `ToStableHashKey`, but then sorting needs to have access to"] # [doc = " a stable hashing context and `ToStableHashKey` can also be expensive as in"] # [doc = " the case of `Symbol` where it has to allocate a `String`.)"] # [doc = ""] # [doc = " See the documentation of [StableOrd] for how stable sort order is defined."] # [doc = " The same definition applies here. Be careful when implementing this trait."] pub trait StableCompare { const CAN_USE_UNSTABLE_SORT : bool ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering ; }
};
}
