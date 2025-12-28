macro_rules! __require_serde_not_serde_core {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! __require_serde_not_serde_core { () => { :: core :: compile_error ! ("Serde derive requires a dependency on the serde crate, not serde_core") ; } ; }
    };
}

__require_serde_not_serde_core!();