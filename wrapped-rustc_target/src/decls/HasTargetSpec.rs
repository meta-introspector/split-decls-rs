macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! HasTargetSpec {
    () => {
        deps!();
        pub trait HasTargetSpec { fn target_spec (& self) -> & Target ; }
    };
}

HasTargetSpec!()