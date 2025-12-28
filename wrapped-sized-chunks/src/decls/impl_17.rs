macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < A , T > Deref for InlineArray < A , T > { type Target = [A] ; fn deref (& self) -> & Self :: Target { unsafe { from_raw_parts (self . data () , self . len ()) } } }
    };
}

impl_17!();