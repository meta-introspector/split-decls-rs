macro_rules! deps {
    () => {
        Effect!();
    };
}

macro_rules! EffectIndex {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct EffectIndex { statement_index : usize , effect : Effect , }
    };
}

EffectIndex!();