macro_rules! deps {
    () => {
        Scope!();
        DropKind!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl Scope { # [doc = " Whether there's anything to do for the cleanup path, that is,"] # [doc = " when unwinding through this scope. This includes destructors,"] # [doc = " but not StorageDead statements, which don't get emitted at all"] # [doc = " for unwinding, for several reasons:"] # [doc = "  * clang doesn't emit llvm.lifetime.end for C++ unwinding"] # [doc = "  * LLVM's memory dependency analysis can't handle it atm"] # [doc = "  * polluting the cleanup MIR with StorageDead creates"] # [doc = "    landing pads even though there's no actual destructors"] # [doc = "  * freeing up stack space has no effect during unwinding"] # [doc = " Note that for coroutines we do emit StorageDeads, for the"] # [doc = " use of optimizations in the MIR coroutine transform."] fn needs_cleanup (& self) -> bool { self . drops . iter () . any (| drop | match drop . kind { DropKind :: Value | DropKind :: ForLint (_) => true , DropKind :: Storage => false , }) } fn invalidate_cache (& mut self) { self . cached_unwind_block = None ; self . cached_coroutine_drop_block = None ; } }
    };
}

impl_151!();