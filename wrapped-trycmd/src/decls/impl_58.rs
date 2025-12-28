macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Spawn { fn is_ok (& self) -> bool { self . status . is_ok () } }
    };
}

impl_58!()