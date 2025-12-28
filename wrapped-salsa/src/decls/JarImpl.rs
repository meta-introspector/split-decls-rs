macro_rules! deps {
    () => {
        Configuration!();
    };
}

macro_rules! JarImpl {
    () => {
        deps!();
        pub struct JarImpl < C > where C : Configuration , { phantom : PhantomData < C > , }
    };
}

JarImpl!()