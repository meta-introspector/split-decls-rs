macro_rules! deps {
    () => {
        StepCursor!();
        Cursor!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < 'c , 'a > Deref for StepCursor < 'c , 'a > { type Target = Cursor < 'c > ; fn deref (& self) -> & Self :: Target { & self . cursor } }
    };
}

impl_504!();