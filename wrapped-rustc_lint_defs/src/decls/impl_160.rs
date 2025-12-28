macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl StableCompare for LintId { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . lint_name_raw () . cmp (& other . lint_name_raw ()) } }
    };
}

impl_160!()