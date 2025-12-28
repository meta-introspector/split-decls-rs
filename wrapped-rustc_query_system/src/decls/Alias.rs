macro_rules! Alias {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum Alias { # [note (query_system_cycle_recursive_ty_alias)] # [help (query_system_cycle_recursive_ty_alias_help1)] # [help (query_system_cycle_recursive_ty_alias_help2)] Ty , # [note (query_system_cycle_recursive_trait_alias)] Trait , }
    };
}

Alias!()