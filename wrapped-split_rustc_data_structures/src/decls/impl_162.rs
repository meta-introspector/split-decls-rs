macro_rules! deps {
    () => {
        MinMaxIn!();
        MinMaxes!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl MinMaxes { fn annotation (& self , scc : usize) -> MinMaxIn { self . 0 [scc] } }
    };
}

impl_162!()