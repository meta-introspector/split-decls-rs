// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_services_fsimpl_1239 {
() => {
// Module: crate::services::fs
// Provides: {"impl_1239"}
// Dependencies: {}
impl < T > AsyncReadBody < T > where T : AsyncRead , { # [doc = " Create a new [`AsyncReadBody`] wrapping the given reader,"] # [doc = " with a specific read buffer capacity"] fn with_capacity (read : T , capacity : usize) -> Self { Self { reader : ReaderStream :: with_capacity (read , capacity) , } } fn with_capacity_limited (read : T , capacity : usize , max_read_bytes : u64 ,) -> AsyncReadBody < Take < T > > { AsyncReadBody { reader : ReaderStream :: with_capacity (read . take (max_read_bytes) , capacity) , } } }
};
}
