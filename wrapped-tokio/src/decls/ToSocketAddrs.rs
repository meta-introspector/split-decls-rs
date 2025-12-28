macro_rules! ToSocketAddrs {
    () => {
        # [doc = " Converts or resolves without blocking to one or more `SocketAddr` values."] # [doc = ""] # [doc = " # DNS"] # [doc = ""] # [doc = " Implementations of `ToSocketAddrs` for string types require a DNS lookup."] # [doc = ""] # [doc = " # Calling"] # [doc = ""] # [doc = " Currently, this trait is only used as an argument to Tokio functions that"] # [doc = " need to reference a target socket address. To perform a `SocketAddr`"] # [doc = " conversion directly, use [`lookup_host()`](super::lookup_host())."] # [doc = ""] # [doc = " This trait is sealed and is intended to be opaque. The details of the trait"] # [doc = " will change. Stabilization is pending enhancements to the Rust language."] pub trait ToSocketAddrs : sealed :: ToSocketAddrsPriv { }
    };
}

ToSocketAddrs!()