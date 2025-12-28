macro_rules! deps {
    () => {
        JarImpl!();
        Configuration!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < C : Configuration > Default for JarImpl < C > { fn default () -> Self { Self { phantom : PhantomData , } } }
    };
}

impl_187!();