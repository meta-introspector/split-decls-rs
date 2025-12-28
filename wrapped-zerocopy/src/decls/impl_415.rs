macro_rules! deps {
    () => {
        Unalign!();
        Unaligned!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < T : Unaligned + PartialOrd > PartialOrd < Unalign < T > > for Unalign < T > { # [inline (always)] fn partial_cmp (& self , other : & Unalign < T >) -> Option < Ordering > { PartialOrd :: partial_cmp (self . deref () , other . deref ()) } }
    };
}

impl_415!()