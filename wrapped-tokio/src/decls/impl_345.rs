macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < T > DerefMut for CachePadded < T > { fn deref_mut (& mut self) -> & mut T { & mut self . value } }
    };
}

impl_345!()