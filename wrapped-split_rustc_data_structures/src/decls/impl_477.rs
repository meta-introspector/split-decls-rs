macro_rules! deps {
    () => {
        StableOrd!();
        StableCompare!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        # [doc = " `StableOrd` denotes that the type's `Ord` implementation is stable, so"] # [doc = " we can implement `StableCompare` by just delegating to `Ord`."] impl < T : StableOrd > StableCompare for T { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . cmp (other) } }
    };
}

impl_477!()