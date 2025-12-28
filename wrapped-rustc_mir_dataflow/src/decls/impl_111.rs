macro_rules! deps {
    () => {
        Effect!();
        EffectIndex!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl Effect { const fn at_index (self , statement_index : usize) -> EffectIndex { EffectIndex { effect : self , statement_index } } }
    };
}

impl_111!()