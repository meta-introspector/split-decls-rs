macro_rules! deps {
    () => {
        StepCursor!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < 'c , 'a > Clone for StepCursor < 'c , 'a > { fn clone (& self) -> Self { * self } }
    };
}

impl_506!()