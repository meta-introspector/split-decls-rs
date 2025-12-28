macro_rules! deps {
    () => {
        Error!();
        Mmap!();
        OwnedSlice!();
        SelfProfiler!();
    };
}

macro_rules! macro_266 {
    () => {
        deps!();
        already_send ! ([std :: backtrace :: Backtrace] [std :: io :: Stdout] [std :: io :: Stderr] [std :: io :: Error] [std :: fs :: File] [rustc_arena :: DroplessArena] [jobserver_crate :: Client] [jobserver_crate :: HelperThread] [crate :: memmap :: Mmap] [crate :: profiling :: SelfProfiler] [crate :: owned_slice :: OwnedSlice]) ;
    };
}

macro_266!()