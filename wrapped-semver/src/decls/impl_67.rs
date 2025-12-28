macro_rules! deps {
    () => {
        Op!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Op { const DEFAULT : Self = Op :: Caret ; }
    };
}

impl_67!();