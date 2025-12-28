macro_rules! deps {
    () => {
        HasTargetSpec!();
        Target!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl HasTargetSpec for Target { # [inline] fn target_spec (& self) -> & Target { self } }
    };
}

impl_532!()