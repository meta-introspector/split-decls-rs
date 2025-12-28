macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl < T : Unaligned + Ord > Ord for Unalign < T > { # [inline (always)] fn cmp (& self , other : & Unalign < T >) -> Ordering { Ord :: cmp (self . deref () , other . deref ()) } }
    };
}

impl_416!();