macro_rules! CouldntDumpMonoStats {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_couldnt_dump_mono_stats)] pub (crate) struct CouldntDumpMonoStats { pub error : String , }
    };
}

CouldntDumpMonoStats!()