macro_rules! deps {
    () => {
        Def!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Def for ! { fn has_safety_invariants (& self) -> bool { unreachable ! () } }
    };
}

impl_31!()