macro_rules! ExternAbiWrapper {
    () => {
        # [doc = " `ExternAbi` is in `rustc_abi`, which doesn't have access to the macro and serde."] struct ExternAbiWrapper (rustc_abi :: ExternAbi) ;
    };
}

ExternAbiWrapper!();