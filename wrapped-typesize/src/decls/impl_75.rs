macro_rules! deps {
    () => {
        SizableArc!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T , SC : ShouldCountInner > core :: ops :: Deref for SizableArc < T , SC > { type Target = Arc < T > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_75!()