macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! define_uuid_macro {
    () => {
        deps!();
        macro_rules ! define_uuid_macro { { $ (# [$ doc : meta]) * } => { $ (# [$ doc]) * # [cfg (feature = "macro-diagnostics")] # [macro_export] macro_rules ! uuid { ($ uuid : expr) => { { const OUTPUT : $ crate :: Uuid = match $ crate :: Uuid :: try_parse ($ uuid) { $ crate :: __macro_support :: Ok (u) => u , $ crate :: __macro_support :: Err (_) => panic ! ("invalid UUID") , } ; OUTPUT } } ; ($ uuid : literal) => { { $ crate :: Uuid :: from_bytes ($ crate :: uuid_macro_internal :: parse_lit ! ($ uuid)) } } ; } $ (# [$ doc]) * # [cfg (not (feature = "macro-diagnostics"))] # [macro_export] macro_rules ! uuid { ($ uuid : expr) => { { const OUTPUT : $ crate :: Uuid = match $ crate :: Uuid :: try_parse ($ uuid) { $ crate :: __macro_support :: Ok (u) => u , $ crate :: __macro_support :: Err (_) => panic ! ("invalid UUID") , } ; OUTPUT } } ; } } }
    };
}

define_uuid_macro!()