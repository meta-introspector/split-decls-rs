macro_rules! deps {
    () => {
        StepCursor!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < 'c , 'a > Copy for StepCursor < 'c , 'a > { }
    };
}

impl_505!();