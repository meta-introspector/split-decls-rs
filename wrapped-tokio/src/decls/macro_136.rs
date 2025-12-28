macro_rules! macro_136 {
    () => {
        cfg_io_driver_impl ! { pub (crate) mod interest ; pub (crate) mod ready ; cfg_net_or_uring ! { pub use interest :: Interest ; pub use ready :: Ready ; } # [cfg_attr (target_os = "wasi" , allow (unused_imports))] mod poll_evented ; # [cfg (not (loom))] # [cfg_attr (target_os = "wasi" , allow (unused_imports))] pub (crate) use poll_evented :: PollEvented ; }
    };
}

macro_136!();