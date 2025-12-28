macro_rules! deps {
    () => {
        OwnedSlice!();
        SelfProfiler!();
        Mmap!();
        Error!();
    };
}

macro_rules! macro_273 {
    () => {
        deps!();
        already_sync ! ([std :: sync :: atomic :: AtomicBool] [std :: sync :: atomic :: AtomicUsize] [std :: sync :: atomic :: AtomicU8] [std :: sync :: atomic :: AtomicU32] [std :: backtrace :: Backtrace] [std :: io :: Error] [std :: fs :: File] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;
    };
}

macro_273!();