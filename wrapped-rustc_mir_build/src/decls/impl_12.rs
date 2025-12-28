macro_rules! deps {
    () => {
        GuardFrameLocal!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl GuardFrameLocal { fn new (id : LocalVarId) -> Self { GuardFrameLocal { id } } }
    };
}

impl_12!();