macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < HCX > ToStableHashKey < HCX > for LintId { type KeyType = & 'static str ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> & 'static str { self . lint_name_raw () } }
    };
}

impl_23!()