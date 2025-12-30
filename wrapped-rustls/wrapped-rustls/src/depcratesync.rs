// Generated macro for sync (module)
macro_rules! Depcratesync {
() => {
// Module: crate
// Provides: {"sync"}
// Dependencies: {}
# [doc = " This internal `sync` module aliases the `Arc` implementation to allow downstream forks"] # [doc = " of rustls targeting architectures without atomic pointers to replace the implementation"] # [doc = " with another implementation such as `portable_atomic_util::Arc` in one central location."] mod sync { # [expect (clippy :: disallowed_types)] pub (crate) type Arc < T > = alloc :: sync :: Arc < T > ; # [expect (clippy :: disallowed_types)] pub (crate) type Weak < T > = alloc :: sync :: Weak < T > ; }
};
}
