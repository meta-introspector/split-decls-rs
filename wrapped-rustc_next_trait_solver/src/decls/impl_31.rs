macro_rules! deps {
    () => {
        SolverDelegate!();
        EagerResolver!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , D : SolverDelegate > EagerResolver < 'a , D > { fn new (delegate : & 'a D) -> Self { EagerResolver { delegate , cache : Default :: default () } } }
    };
}

impl_31!();