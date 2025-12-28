macro_rules! deps {
    () => {
        Configuration!();
        JarImpl!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < C : Configuration > Default for JarImpl < C > { fn default () -> Self { Self { phantom : Default :: default () , } } }
    };
}

impl_353!();