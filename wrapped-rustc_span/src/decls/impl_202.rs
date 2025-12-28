macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl StableCompare for Symbol { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
    };
}

impl_202!()