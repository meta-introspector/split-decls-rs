macro_rules! deps {
    () => {
        Accumulator!();
        JarImpl!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < A : Accumulator > Default for JarImpl < A > { fn default () -> Self { Self { phantom : Default :: default () , } } }
    };
}

impl_4!();