macro_rules! deps {
    () => {
        PatternExtraData!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'tcx > PatternExtraData < 'tcx > { fn is_empty (& self) -> bool { self . bindings . is_empty () && self . ascriptions . is_empty () } }
    };
}

impl_108!();