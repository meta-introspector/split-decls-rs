macro_rules! TransferFunction {
    () => {
        pub struct TransferFunction < 'a > (pub & 'a mut DenseBitSet < Local >) ;
    };
}

TransferFunction!()