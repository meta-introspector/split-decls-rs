macro_rules! deps {
    () => {
        IntoDynSyncSend!();
    };
}

macro_rules! DynSend {
    () => {
        deps!();
        # [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSend`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Send`")] pub unsafe auto trait DynSend { }
    };
}

DynSend!()