macro_rules! DebuginfoLocals {
    () => {
        struct DebuginfoLocals (DenseBitSet < Local >) ;
    };
}

DebuginfoLocals!()