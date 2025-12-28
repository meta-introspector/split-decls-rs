macro_rules! deps {
    () => {
        JodChild!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ops :: DerefMut for JodChild { fn deref_mut (& mut self) -> & mut std :: process :: Child { & mut self . 0 } }
    };
}

impl_28!()