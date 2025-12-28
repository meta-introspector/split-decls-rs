macro_rules! deps {
    () => {
        Unalign!();
        Unaligned!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl < T : Unaligned > DerefMut for Unalign < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { Ptr :: from_mut (self) . transmute :: < _ , _ , (_ , (_ , _)) > () . bikeshed_recall_aligned () . as_mut () } }
    };
}

impl_414!();