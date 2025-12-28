macro_rules! deps {
    () => {
        Transitions!();
        Region!();
        Type!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < R , T > Default for Transitions < R , T > where R : Region , T : Type , { fn default () -> Self { Self { byte_transitions : EdgeSet :: empty () , ref_transitions : Map :: default () } } }
    };
}

impl_9!();