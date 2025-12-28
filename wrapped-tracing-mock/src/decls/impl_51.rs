macro_rules! deps {
    () => {
        ActualSpan!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < & tracing_core :: span :: Id > for ActualSpan { fn from (id : & tracing_core :: span :: Id) -> Self { Self :: new (id . clone () , None) } }
    };
}

impl_51!();