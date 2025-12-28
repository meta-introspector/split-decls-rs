macro_rules! deps {
    () => {
        JarImpl!();
        Configuration!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < C : Configuration > Default for JarImpl < C > { fn default () -> Self { Self { _phantom : Default :: default () , } } }
    };
}

impl_160!();