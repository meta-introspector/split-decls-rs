macro_rules! deps {
    () => {
        MinMaxes!();
        MinMaxIn!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl MinMaxes { fn annotation (& self , scc : usize) -> MinMaxIn { self . 0 [scc] } }
    };
}

impl_162!();