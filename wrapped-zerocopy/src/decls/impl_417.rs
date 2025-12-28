macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < T : Unaligned + PartialEq > PartialEq < Unalign < T > > for Unalign < T > { # [inline (always)] fn eq (& self , other : & Unalign < T >) -> bool { PartialEq :: eq (self . deref () , other . deref ()) } }
    };
}

impl_417!()