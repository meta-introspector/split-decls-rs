macro_rules! StartNotFound {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_start_not_found)] # [help] pub (crate) struct StartNotFound ;
    };
}

StartNotFound!()