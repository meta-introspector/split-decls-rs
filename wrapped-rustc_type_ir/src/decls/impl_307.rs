macro_rules! deps {
    () => {
        Interner!();
        Shifter!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < I : Interner > Shifter < I > { fn new (cx : I , amount : u32) -> Self { Shifter { cx , current_index : ty :: INNERMOST , amount } } }
    };
}

impl_307!()