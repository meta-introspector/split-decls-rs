macro_rules! deps {
    () => {
        Variables!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl DerefMut for Variables { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_81!();