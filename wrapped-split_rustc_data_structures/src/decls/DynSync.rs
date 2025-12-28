macro_rules! deps {
    () => {
        IntoDynSyncSend!();
    };
}

macro_rules! DynSync {
    () => {
        deps!();
        # [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSync`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")] pub unsafe auto trait DynSync { }
    };
}

DynSync!();