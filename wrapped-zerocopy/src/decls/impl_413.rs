macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < T : Unaligned > Deref for Unalign < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { Ptr :: from_ref (self) . transmute () . bikeshed_recall_aligned () . as_ref () } }
    };
}

impl_413!();