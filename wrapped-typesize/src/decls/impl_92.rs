macro_rules! impl_92 {
    () => {
        impl < T : ? Sized > DerefMut for RefMut < '_ , T > { fn deref_mut (& mut self) -> & mut Self :: Target { self . 0 } }
    };
}

impl_92!()