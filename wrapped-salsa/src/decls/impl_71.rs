macro_rules! deps {
    () => {
        ProvisionalStatus!();
        CycleHeads!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'db > ProvisionalStatus < 'db > { pub (crate) fn cycle_heads (& self) -> & 'db CycleHeads { match self { ProvisionalStatus :: Provisional { cycle_heads , .. } => cycle_heads , _ => empty_cycle_heads () , } } pub (crate) const fn is_provisional (& self) -> bool { matches ! (self , ProvisionalStatus :: Provisional { .. }) } }
    };
}

impl_71!()