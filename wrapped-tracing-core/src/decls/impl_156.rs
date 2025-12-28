macro_rules! deps {
    () => {
        DebugValue!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T : fmt :: Debug > crate :: sealed :: Sealed for DebugValue < T > { }
    };
}

impl_156!();