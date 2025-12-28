macro_rules! deps {
    () => {
        SizableRc!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T , SC : ShouldCountInner > core :: ops :: Deref for SizableRc < T , SC > { type Target = Rc < T > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_82!();