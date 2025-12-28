macro_rules! deps {
    () => {
        NormalizedPos!();
    };
}

macro_rules! normalize_src {
    () => {
        deps!();
        # [doc = " Normalizes the source code and records the normalizations."] fn normalize_src (src : & mut String) -> Vec < NormalizedPos > { let mut normalized_pos = vec ! [] ; remove_bom (src , & mut normalized_pos) ; normalize_newlines (src , & mut normalized_pos) ; normalized_pos }
    };
}

normalize_src!()