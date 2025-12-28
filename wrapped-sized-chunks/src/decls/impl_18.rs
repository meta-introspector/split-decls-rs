macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < A , T > DerefMut for InlineArray < A , T > { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { from_raw_parts_mut (self . data_mut () , self . len ()) } } }
    };
}

impl_18!()