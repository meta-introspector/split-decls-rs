macro_rules! deps {
    () => {
        CustomSpawn!();
        ThreadBuilder!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < F > CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { pub (super) fn new (spawn : F) -> Self { CustomSpawn (spawn) } }
    };
}

impl_105!()